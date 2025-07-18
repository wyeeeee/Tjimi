use axum::{
    Json, RequestExt,
    extract::{FromRequest, Path, Request},
};

use crate::{
    config::CLEWDR_CONFIG,
    error::ClewdrError,
    gemini_body::GeminiArgs,
    gemini_state::{GeminiApiFormat, GeminiState},
    types::{claude_message::CreateMessageParams, gemini::request::GeminiRequestBody},
};

pub struct GeminiContext {
    pub model: String,
    pub vertex: bool,
    pub stream: bool,
    pub path: String,
    pub query: GeminiArgs,
    pub api_format: GeminiApiFormat,
}

pub struct GeminiPreprocess(pub GeminiRequestBody, pub GeminiContext);

impl FromRequest<GeminiState> for GeminiPreprocess {
    type Rejection = ClewdrError;

    async fn from_request(mut req: Request, state: &GeminiState) -> Result<Self, Self::Rejection> {
        let Path(path) = req.extract_parts::<Path<String>>().await?;
        let vertex = req.uri().to_string().contains("vertex");
        if vertex && !CLEWDR_CONFIG.load().vertex.validate() {
            return Err(ClewdrError::BadRequest {
                msg: "Vertex is not configured",
            });
        }
        let mut model = path
            .split('/')
            .next_back()
            .map(|s| s.split_once(':').map(|s| s.0).unwrap_or(s).to_string());
        if vertex {
            model = CLEWDR_CONFIG.load().vertex.model_id.to_owned().or(model)
        }
        let Some(model) = model else {
            return Err(ClewdrError::BadRequest {
                msg: "Model not found in path or vertex config",
            });
        };
        let query = req.extract_parts::<GeminiArgs>().await?;
        let ctx = GeminiContext {
            vertex,
            model,
            stream: path.contains("streamGenerateContent"),
            path,
            query,
            api_format: GeminiApiFormat::Gemini,
        };
        let Json(mut body) = Json::<GeminiRequestBody>::from_request(req, &()).await?;
        body.safety_off();
        let mut state = state.clone();
        state.update_from_ctx(&ctx);
        if let Some(res) = state.try_from_cache(&body).await {
            return Err(ClewdrError::CacheFound { res: Box::new(res) });
        }
        Ok(GeminiPreprocess(body, ctx))
    }
}

pub struct GeminiOaiPreprocess(pub CreateMessageParams, pub GeminiContext);

impl FromRequest<GeminiState> for GeminiOaiPreprocess {
    type Rejection = ClewdrError;

    async fn from_request(req: Request, state: &GeminiState) -> Result<Self, Self::Rejection> {
        let vertex = req.uri().to_string().contains("vertex");
        if vertex && !CLEWDR_CONFIG.load().vertex.validate() {
            return Err(ClewdrError::BadRequest {
                msg: "Vertex is not configured",
            });
        }
        let Json(mut body) = Json::<CreateMessageParams>::from_request(req, &()).await?;
        let model = body.model.to_owned();
        if vertex {
            body.preprocess_vertex();
        }
        let stream = body.stream.unwrap_or_default();
        let ctx = GeminiContext {
            vertex,
            model,
            stream,
            path: String::new(),
            query: GeminiArgs::default(),
            api_format: GeminiApiFormat::OpenAI,
        };
        let mut state = state.clone();
        state.update_from_ctx(&ctx);
        if let Some(res) = state.try_from_cache(&body).await {
            return Err(ClewdrError::CacheFound { res: Box::new(res) });
        }
        Ok(GeminiOaiPreprocess(body, ctx))
    }
}
