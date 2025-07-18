use crate::error::ClewdrError;
use axum::extract::{FromRequestParts, Query};
use serde::Deserialize;
use struct_iterable::Iterable;

#[derive(Debug, Clone, Deserialize, Iterable, Default)]
pub struct GeminiArgs {
    pub key: String,
    pub alt: Option<String>,
}

#[derive(Deserialize)]
struct GeminiQueryAlt {
    pub alt: Option<String>,
}

impl<S> FromRequestParts<S> for GeminiArgs
where
    S: Sync,
{
    type Rejection = ClewdrError;

    async fn from_request_parts(
        parts: &mut axum::http::request::Parts,
        _: &S,
    ) -> Result<Self, Self::Rejection> {
        match Query::<GeminiArgs>::from_request_parts(parts, &()).await {
            Ok(Query(q)) => Ok(q),
            Err(_) => {
                let Query(q) = Query::<GeminiQueryAlt>::from_request_parts(parts, &()).await?;
                // extract key from x-goog-api-key
                let key = parts
                    .headers
                    .get("x-goog-api-key")
                    .and_then(|v| v.to_str().ok())
                    .ok_or(ClewdrError::InvalidKey)?;
                Ok(GeminiArgs {
                    key: key.to_string(),
                    alt: q.alt,
                })
            }
        }
    }
}

impl GeminiArgs {
    pub fn to_vec(&self) -> Vec<(&'static str, &str)> {
        let mut vec = Vec::new();
        for (k, vv) in self.iter() {
            if k == "key" {
                continue;
            }
            if let Some(v) = vv
                .downcast_ref::<String>()
                .or(vv.downcast_ref::<Option<String>>().and_then(|v| v.as_ref()))
            {
                vec.push((k, v.as_str()));
            }
        }
        vec
    }
}
