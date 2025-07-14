use axum::{http::StatusCode, response::Json};
use serde_json::{json, Value};

pub async fn health_check() -> Result<Json<Value>, StatusCode> {
    Ok(Json(json!({
        "status": "healthy",
        "timestamp": chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true),
        "service": "Gemini Proxy"
    })))
}