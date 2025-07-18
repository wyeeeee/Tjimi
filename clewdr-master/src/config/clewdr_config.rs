use axum::http::{Uri, uri::Scheme};
use colored::Colorize;
use figment::{
    Figment,
    providers::{Env, Format, Toml},
};
use http::uri::Authority;
use passwords::PasswordGenerator;
use serde::{Deserialize, Serialize};
use std::{
    collections::HashSet,
    env,
    fmt::{Debug, Display},
    net::{IpAddr, SocketAddr},
    path::PathBuf,
    sync::Arc,
};
use tiktoken_rs::o200k_base;
use tokio::spawn;
use tracing::{error, warn};
use wreq::{Proxy, Url};
use yup_oauth2::ServiceAccountKey;

use crate::{
    config::{
        CC_CLIENT_ID, CookieStatus, UselessCookie, default_check_update, default_ip,
        default_max_retries, default_padtxt_len, default_port, default_skip_cool_down,
        default_use_real_roles,
    },
    error::ClewdrError,
    utils::enabled,
};

use super::{ARG_COOKIE_FILE, CONFIG_PATH, ENDPOINT_URL, key::KeyStatus};

/// Generates a random password for authentication
/// Creates a secure 64-character password with mixed character types
///
/// # Returns
/// A random password string
fn generate_password() -> String {
    let pg = PasswordGenerator {
        length: 64,
        numbers: true,
        lowercase_letters: true,
        uppercase_letters: true,
        symbols: false,
        spaces: false,
        exclude_similar_characters: true,
        strict: true,
    };

    println!("{}", "Generating random password......".green());
    pg.generate_one().unwrap()
}

#[derive(Debug, Serialize, Deserialize, Clone, Default)]
pub struct VertexConfig {
    #[serde(default)]
    pub credential: Option<ServiceAccountKey>,
    pub model_id: Option<String>,
}

impl VertexConfig {
    pub fn validate(&self) -> bool {
        self.credential.is_some()
    }
}

/// A struct representing the configuration of the application
#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct ClewdrConfig {
    // key configurations
    #[serde(default)]
    pub vertex: VertexConfig,
    #[serde(default)]
    pub cookie_array: HashSet<CookieStatus>,
    #[serde(default)]
    pub wasted_cookie: HashSet<UselessCookie>,
    #[serde(default)]
    pub gemini_keys: HashSet<KeyStatus>,

    // Server settings, cannot hot reload
    #[serde(default = "default_ip")]
    ip: IpAddr,
    #[serde(default = "default_port")]
    port: u16,

    // App settings, can hot reload, but meaningless
    #[serde(default = "default_check_update")]
    pub check_update: bool,
    #[serde(default)]
    pub auto_update: bool,

    // Network settings, can hot reload
    #[serde(default)]
    password: String,
    #[serde(default)]
    admin_password: String,
    #[serde(default)]
    pub proxy: Option<String>,
    #[serde(default)]
    pub rproxy: Option<Url>,

    // Api settings, can hot reload
    #[serde(default = "default_max_retries")]
    pub max_retries: usize,
    #[serde(default)]
    pub preserve_chats: bool,
    #[serde(default)]
    pub web_search: bool,

    // Cache settings, can hot reload
    #[serde(default)]
    pub cache_response: usize,
    #[serde(default)]
    pub not_hash_system: bool,
    #[serde(default)]
    pub not_hash_last_n: usize,

    // Cookie settings, can hot reload
    #[serde(default)]
    pub skip_first_warning: bool,
    #[serde(default)]
    pub skip_second_warning: bool,
    #[serde(default)]
    pub skip_restricted: bool,
    #[serde(default)]
    pub skip_non_pro: bool,
    #[serde(default = "default_skip_cool_down")]
    pub skip_rate_limit: bool,
    #[serde(default)]
    pub skip_normal_pro: bool,

    // Prompt configurations, can hot reload
    #[serde(default = "default_use_real_roles")]
    pub use_real_roles: bool,
    #[serde(default)]
    pub custom_h: Option<String>,
    #[serde(default)]
    pub custom_a: Option<String>,
    #[serde(default)]
    pub custom_prompt: String,
    #[serde(default)]
    pub padtxt_file: Option<PathBuf>,
    #[serde(default = "default_padtxt_len")]
    pub padtxt_len: usize,

    // Claude Code settings, can hot reload
    #[serde(default)]
    pub claude_code_client_id: Option<String>,
    #[serde(default)]
    pub custom_system: Option<String>,

    // Skip field, can hot reload
    #[serde(skip)]
    pub rquest_proxy: Option<Proxy>,
    #[serde(skip)]
    pub pad_tokens: Arc<Vec<String>>,
}

impl Default for ClewdrConfig {
    fn default() -> Self {
        Self {
            vertex: Default::default(),
            max_retries: default_max_retries(),
            check_update: default_check_update(),
            auto_update: false,
            cookie_array: HashSet::new(),
            wasted_cookie: HashSet::new(),
            gemini_keys: HashSet::new(),
            password: String::new(),
            admin_password: String::new(),
            proxy: None,
            ip: default_ip(),
            port: default_port(),
            rproxy: None,
            use_real_roles: default_use_real_roles(),
            custom_prompt: String::new(),
            padtxt_file: None,
            padtxt_len: default_padtxt_len(),
            custom_h: None,
            custom_a: None,
            rquest_proxy: None,
            pad_tokens: Arc::new(vec![]),
            preserve_chats: false,
            web_search: false,
            cache_response: 0,
            not_hash_system: false,
            not_hash_last_n: 0,
            skip_first_warning: false,
            skip_second_warning: false,
            skip_restricted: false,
            skip_non_pro: false,
            skip_rate_limit: default_skip_cool_down(),
            skip_normal_pro: false,
            claude_code_client_id: None,
            custom_system: None,
        }
    }
}

impl Display for ClewdrConfig {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        // one line per field
        let authority = self.address();
        let authority: Authority = authority.to_string().parse().map_err(|_| std::fmt::Error)?;
        let api_url = Uri::builder()
            .scheme(Scheme::HTTP)
            .authority(authority.to_owned())
            .path_and_query("/v1")
            .build()
            .map_err(|_| std::fmt::Error)?;
        let web_url = Uri::builder()
            .scheme(Scheme::HTTP)
            .authority(authority.to_string())
            .path_and_query("")
            .build()
            .map_err(|_| std::fmt::Error)?;
        write!(
            f,
            "Claude(Claude and OpenAI format) / Gemini(Gemini format) Endpoint: {}\n\
            Claude Code(Claude and OpenAI format) Endpoint: {}\n\
            Vertex(Gemini format) Endpoint: {}\n\
            Gemini(OpenAI format) Endpoint: {}\n\
            Vertex(OpenAI format) Endpoint: {}\n\
            API Password: {}\n\
            Web Admin Endpoint: {}\n\
            Web Admin Password: {}\n",
            api_url.to_string().green().underline(),
            (web_url.to_string() + "code/v1").green().underline(),
            (api_url.to_string() + "/vertex").green().underline(),
            (web_url.to_string() + "gemini").green().underline(),
            (web_url.to_string() + "gemini/vertex").green().underline(),
            self.password.yellow(),
            web_url.to_string().green().underline(),
            self.admin_password.yellow(),
        )?;
        writeln!(
            f,
            "Response Caching: {}",
            self.cache_response.to_string().green()
        )?;
        if let Some(ref proxy) = self.proxy {
            writeln!(f, "Proxy: {}", proxy.to_string().blue())?;
        }
        if let Some(ref rproxy) = self.rproxy {
            writeln!(f, "Reverse Proxy: {}", rproxy.to_string().blue())?;
        }
        if !self.pad_tokens.is_empty() {
            writeln!(
                f,
                "Pad txt token count: {}",
                self.pad_tokens.len().to_string().blue()
            )?
        }
        if self.vertex.validate() {
            writeln!(f, "Vertex {}", "Enabled".green().bold())?;
        }
        writeln!(f, "Skip non Pro: {}", enabled(self.skip_non_pro))?;
        writeln!(f, "Skip restricted: {}", enabled(self.skip_restricted))?;
        writeln!(
            f,
            "Skip second warning: {}",
            enabled(self.skip_second_warning)
        )?;
        writeln!(
            f,
            "Skip first warning: {}",
            enabled(self.skip_first_warning)
        )?;
        writeln!(f, "Skip normal Pro: {}", enabled(self.skip_normal_pro))?;
        writeln!(f, "Skip rate limit: {}", enabled(self.skip_rate_limit))?;
        Ok(())
    }
}

impl ClewdrConfig {
    pub fn user_auth(&self, key: &str) -> bool {
        key == self.password
    }

    pub fn admin_auth(&self, key: &str) -> bool {
        key == self.admin_password
    }

    pub fn cc_client_id(&self) -> String {
        self.claude_code_client_id
            .as_deref()
            .unwrap_or(CC_CLIENT_ID)
            .to_string()
    }

    /// Loads configuration from files and environment variables
    /// Combines settings from config.toml, clewdr.toml, and environment variables
    /// Also loads cookies from a file if specified
    ///
    /// # Returns
    /// * Config instance
    pub fn new() -> Self {
        let mut config: ClewdrConfig = Figment::from(Toml::file(CONFIG_PATH.as_path()))
            .admerge(Env::prefixed("CLEWDR_"))
            .extract_lossy()
            .inspect_err(|e| {
                error!("Failed to load config: {}", e);
            })
            .unwrap_or_default();
        if let Some(credential) = env::var("CLEWDR_VERTEX_CREDENTIAL").ok().and_then(|v| {
            serde_json::from_str::<ServiceAccountKey>(&v)
                .map_err(|e| error!("Failed to parse vertex credential: {}", e))
                .ok()
        }) {
            config.vertex.credential = Some(credential);
        }
        if let Some(ref f) = *ARG_COOKIE_FILE {
            // load cookies from file
            if f.exists() {
                if let Ok(cookies) = std::fs::read_to_string(f) {
                    let cookies = cookies
                        .lines()
                        .filter_map(|line| CookieStatus::new(line, None).ok());
                    config.cookie_array.extend(cookies);
                } else {
                    error!("Failed to read cookie file: {}", f.display());
                }
            } else {
                error!("Cookie file not found: {}", f.display());
            }
        }
        let config = config.validate();
        let config_clone = config.to_owned();
        spawn(async move {
            config_clone.save().await.unwrap_or_else(|e| {
                error!("Failed to save config: {}", e);
            });
        });
        config
    }

    /// Loads padding text from a file
    /// Used to pad prompts with tokens to reach minimum token requirements
    ///
    /// # Effects
    /// Updates the pad_tokens field with tokenized content from the file
    fn load_padtxt(&mut self) -> Result<(), ClewdrError> {
        let Some(padtxt) = &self.padtxt_file else {
            self.pad_tokens = Default::default();
            return Ok(());
        };
        if !padtxt.exists() {
            return Err(ClewdrError::PathNotFound {
                msg: format!("Pad txt file not found: {}", padtxt.display()),
            });
        }
        let padtxt_string = std::fs::read_to_string(padtxt.as_path())?;

        let bpe = o200k_base().unwrap();
        let ranks = bpe.encode_with_special_tokens(&padtxt_string);
        let tokens = ranks
            .into_iter()
            .filter_map(|token| bpe.decode(vec![token]).ok())
            .collect::<Vec<_>>();
        if tokens.len() < 4096 {
            warn!(
                "Pad txt file {} is too short, token count {}",
                padtxt.display(),
                tokens.len()
            );
            return Err(ClewdrError::PadtxtTooShort);
        }
        self.pad_tokens = Arc::new(tokens);
        Ok(())
    }

    /// Gets the API endpoint for the Claude service
    /// Returns the reverse proxy URL if configured, otherwise the default endpoint
    ///
    /// # Returns
    /// The URL for the API endpoint
    pub fn endpoint(&self) -> Url {
        if let Some(ref proxy) = self.rproxy {
            return proxy.to_owned();
        }
        ENDPOINT_URL.to_owned()
    }

    /// address of proxy
    pub fn address(&self) -> SocketAddr {
        SocketAddr::new(self.ip, self.port)
    }

    /// Save the configuration to a file
    pub async fn save(&self) -> Result<(), ClewdrError> {
        #[cfg(feature = "no_fs")]
        {
            return Ok(());
        }
        Ok(tokio::fs::write(CONFIG_PATH.as_path(), toml::ser::to_string_pretty(self)?).await?)
    }

    /// Validate the configuration
    pub fn validate(mut self) -> Self {
        const MAX_CACHE_RESPONSE: usize = 20;
        if self.password.trim().is_empty() {
            self.password = generate_password();
        }
        if self.admin_password.trim().is_empty() {
            self.admin_password = generate_password();
        }
        self.cache_response = self.cache_response.min(MAX_CACHE_RESPONSE);
        self.cookie_array = self.cookie_array.into_iter().map(|x| x.reset()).collect();
        self.rquest_proxy = self.proxy.to_owned().and_then(|p| {
            Proxy::all(p)
                .inspect_err(|e| {
                    self.proxy = None;
                    error!("Failed to parse proxy: {}", e);
                })
                .ok()
        });
        self.load_padtxt().unwrap_or_else(|e| {
            error!("Failed to load padtxt: {}", e);
            self.pad_tokens = Default::default();
            self.padtxt_file = None;
        });
        self
    }
}
