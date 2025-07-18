use std::pin::Pin;
use std::{collections::HashMap, str::FromStr};

use oauth2::{
    AsyncHttpClient, AuthUrl, AuthorizationCode, ClientId, CsrfToken, HttpClientError, HttpRequest,
    HttpResponse, PkceCodeChallenge, PkceCodeVerifier, RedirectUrl, Scope, TokenUrl, http,
};
use serde_json::Value;
use snafu::{OptionExt, ResultExt};
use url::Url;

use crate::config::CookieStatus;
use crate::error::UnexpectedNoneSnafu;
use crate::{
    claude_code_state::ClaudeCodeState,
    config::{CC_REDIRECT_URI, CC_TOKEN_URL, CLEWDR_CONFIG, TokenInfo},
    error::{CheckClaudeErr, ClewdrError, RquestSnafu, UrlSnafu},
};

struct OauthClient {
    client: wreq::Client,
}

impl<'c> AsyncHttpClient<'c> for OauthClient {
    type Error = HttpClientError<wreq::Error>;

    type Future =
        Pin<Box<dyn Future<Output = Result<HttpResponse, Self::Error>> + Send + Sync + 'c>>;

    fn call(&'c self, request: HttpRequest) -> Self::Future {
        Box::pin(async move {
            let response = self
                .client
                .execute(request.try_into().map_err(Box::new)?)
                .await
                .map_err(Box::new)?;

            let mut builder = http::Response::builder().status(response.status());

            {
                builder = builder.version(response.version());
            }

            for (name, value) in response.headers().iter() {
                builder = builder.header(name, value);
            }

            builder
                .body(response.bytes().await.map_err(Box::new)?.to_vec())
                .map_err(HttpClientError::Http)
        })
    }
}

pub struct ExchangeResult {
    code: String,
    state: Option<String>,
    verifier: PkceCodeVerifier,
    org_uuid: String,
}

impl ClaudeCodeState {
    pub async fn exchange_code(&self, org_uuid: &str) -> Result<ExchangeResult, ClewdrError> {
        let authorize_url = |org_uuid: &str| {
            format!(
                "{}/v1/oauth/{}/authorize",
                CLEWDR_CONFIG.load().endpoint(),
                org_uuid
            )
        };
        let cc_client_id = CLEWDR_CONFIG.load().cc_client_id();

        let client = oauth2::basic::BasicClient::new(ClientId::new(cc_client_id))
            .set_auth_type(oauth2::AuthType::RequestBody)
            .set_redirect_uri(RedirectUrl::new(CC_REDIRECT_URI.into()).map_err(|_| {
                ClewdrError::UnexpectedNone {
                    msg: "Invalid redirect URI",
                }
            })?)
            .set_auth_uri(AuthUrl::new(authorize_url(org_uuid)).map_err(|_| {
                ClewdrError::UnexpectedNone {
                    msg: "Invalid auth URI",
                }
            })?)
            .set_token_uri(TokenUrl::new(CC_TOKEN_URL.into()).map_err(|_| {
                ClewdrError::UnexpectedNone {
                    msg: "Invalid token URI",
                }
            })?);

        let (pkce_challenge, pkce_verifier) = PkceCodeChallenge::new_random_sha256();

        let (mut auth_url, _csrf_token) = client
            .authorize_url(|| CsrfToken::new_random_len(32))
            .add_scope(Scope::new("user:profile".to_string()))
            .add_scope(Scope::new("user:inference".to_string()))
            .set_pkce_challenge(pkce_challenge)
            .url();

        let mut query_params: HashMap<String, String> =
            auth_url.query_pairs().into_owned().collect();
        query_params.insert("organization_uuid".to_string(), org_uuid.to_string());
        auth_url.set_query(None);

        let wreq_client = self.get_wreq_client();
        let redirect_json = wreq_client
            .post(auth_url)
            .json(&query_params)
            .send()
            .await
            .context(RquestSnafu {
                msg: "Failed to send authorization request",
            })?
            .check_claude()
            .await?
            .json::<Value>()
            .await
            .context(RquestSnafu {
                msg: "Failed to parse authorization response",
            })?;

        let redirect_uri = redirect_json["redirect_uri"]
            .as_str()
            .expect("Expected redirect_uri in response");
        let redirect_url = Url::from_str(redirect_uri).context(UrlSnafu {
            url: redirect_uri.to_string(),
        })?;

        let query = redirect_url.query_pairs().collect::<HashMap<_, _>>();
        let code = query.get("code").context(UnexpectedNoneSnafu {
            msg: "No code found in redirect URL",
        })?;
        let state = query.get("state");

        Ok(ExchangeResult {
            code: code.to_string(),
            state: state.map(|s| s.to_string()),
            verifier: pkce_verifier,
            org_uuid: org_uuid.to_string(),
        })
    }

    pub async fn exchange_token(&mut self, code_res: ExchangeResult) -> Result<(), ClewdrError> {
        let cc_client_id = CLEWDR_CONFIG.load().cc_client_id();

        let client = oauth2::basic::BasicClient::new(ClientId::new(cc_client_id))
            .set_auth_type(oauth2::AuthType::RequestBody)
            .set_redirect_uri(RedirectUrl::new(CC_REDIRECT_URI.into()).map_err(|_| {
                ClewdrError::UnexpectedNone {
                    msg: "Invalid redirect URI",
                }
            })?)
            .set_token_uri(TokenUrl::new(CC_TOKEN_URL.into()).map_err(|_| {
                ClewdrError::UnexpectedNone {
                    msg: "Invalid token URI",
                }
            })?);

        let wreq_client = self.get_wreq_client();
        let my_client = OauthClient {
            client: wreq_client.clone(),
        };

        let mut token_request = client
            .exchange_code(AuthorizationCode::new(code_res.code))
            .set_pkce_verifier(code_res.verifier);

        if let Some(state) = code_res.state {
            token_request = token_request.add_extra_param("state", state);
        }

        let token = token_request.request_async(&my_client).await?;

        if let Some(cookie) = self.cookie.as_mut() {
            cookie.token = Some(TokenInfo::new(token, code_res.org_uuid.clone()));
        } else {
            return Err(ClewdrError::UnexpectedNone {
                msg: "No cookie found to update with token info",
            });
        }
        Ok(())
    }

    pub async fn refresh_token(&mut self) -> Result<(), ClewdrError> {
        let wreq_client = self.get_wreq_client();
        let Some(CookieStatus {
            token: Some(ref mut token),
            ..
        }) = self.cookie
        else {
            return Err(ClewdrError::UnexpectedNone {
                msg: "No token found to refresh token",
            });
        };
        if !token.is_expired() {
            return Ok(());
        }

        let cc_client_id = CLEWDR_CONFIG.load().cc_client_id();

        let client = oauth2::basic::BasicClient::new(ClientId::new(cc_client_id))
            .set_auth_type(oauth2::AuthType::RequestBody)
            .set_token_uri(TokenUrl::new(CC_TOKEN_URL.into()).map_err(|_| {
                ClewdrError::UnexpectedNone {
                    msg: "Invalid token URI",
                }
            })?);

        let my_client = OauthClient {
            client: wreq_client.clone(),
        };

        let new_token = client
            .exchange_refresh_token(&oauth2::RefreshToken::new(token.refresh_token.to_owned()))
            .request_async(&my_client)
            .await?;

        *token = TokenInfo::new(new_token, token.organization.uuid.clone());
        Ok(())
    }

    fn get_wreq_client(&self) -> wreq::Client {
        self.client.clone()
    }
}
