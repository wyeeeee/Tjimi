use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Json, Response, Sse, IntoResponse},
    response::sse::Event,
};
use crate::services::GeminiProxyService;
use serde_json::Value;
use std::sync::Arc;
use sqlx::SqlitePool;
use tokio_stream::StreamExt;

pub async fn list_models(
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<Value>, StatusCode> {
    let proxy_service = GeminiProxyService::new(pool.as_ref().clone());
    
    match proxy_service.forward_request("GET", "/v1/models", serde_json::json!({})).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            tracing::error!("Failed to list models: {}", e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_model(
    Path(model): Path<String>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<Value>, StatusCode> {
    let proxy_service = GeminiProxyService::new(pool.as_ref().clone());
    let path = format!("/v1/models/{}", model);
    
    match proxy_service.forward_request("GET", &path, serde_json::json!({})).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            tracing::error!("Failed to get model {}: {}", model, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn generate_content(
    Path(model): Path<String>,
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<Value>,
) -> Result<Json<Value>, StatusCode> {
    let proxy_service = GeminiProxyService::new(pool.as_ref().clone());
    let path = format!("/v1/models/{}:generateContent", model);
    
    match proxy_service.forward_request("POST", &path, payload).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            tracing::error!("Failed to generate content for model {}: {}", model, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn stream_generate_content(
    Path(model): Path<String>,
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<Value>,
) -> Result<Response, StatusCode> {
    let proxy_service = GeminiProxyService::new(pool.as_ref().clone());
    let path = format!("/v1/models/{}:streamGenerateContent", model);
    
    match proxy_service.forward_streaming_request("POST", &path, payload).await {
        Ok(stream) => {
            let sse_stream = stream.map(|chunk| -> Result<Event, axum::Error> {
                match chunk {
                    Ok(bytes) => {
                        let text = String::from_utf8_lossy(&bytes);
                        // 过滤空行和处理 SSE 格式
                        let lines: Vec<&str> = text.lines().collect();
                        for line in lines {
                            if line.starts_with("data: ") {
                                let data = &line[6..]; // 移除 "data: " 前缀
                                if !data.is_empty() && data != "[DONE]" {
                                    return Ok(Event::default().data(data.to_string()));
                                }
                            }
                        }
                        // 如果没有找到有效的 data 行，返回原始内容
                        Ok(Event::default().data(text.trim().to_string()))
                    }
                    Err(e) => {
                        tracing::error!("Stream error: {}", e);
                        Ok(Event::default().data(format!(r#"{{"error": "{}"}}"#, e)))
                    }
                }
            });
            
            let response = Sse::new(sse_stream)
                .keep_alive(
                    axum::response::sse::KeepAlive::new()
                        .interval(std::time::Duration::from_secs(30))
                        .text("keep-alive-text")
                )
                .into_response();
                
            Ok(response)
        }
        Err(e) => {
            tracing::error!("Failed to stream generate content for model {}: {}", model, e);
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}