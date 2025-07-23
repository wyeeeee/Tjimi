use axum::{
    extract::{Path, State},
    http::StatusCode,
    response::{Json, Response, Sse, IntoResponse},
    response::sse::Event,
};
use crate::services::{GeminiProxyService, ErrorLoggerService};
use serde_json::Value;
use std::sync::Arc;
use std::time::Instant;
use sqlx::SqlitePool;
use tokio_stream::StreamExt;

pub async fn api_info() -> Result<Json<Value>, StatusCode> {
    let info = serde_json::json!({
        "name": "Gemini API Proxy",
        "version": "1.0.0",
        "description": "Proxy service for Google Gemini API",
        "status": "ok",
        "endpoints": [
            "/v1/models",
            "/v1/models/{model}",
            "/v1/models/{model}:generateContent",
            "/v1/models/{model}:streamGenerateContent"
        ]
    });
    Ok(Json(info))
}

pub async fn list_models(
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<Value>, StatusCode> {
    let start_time = Instant::now();
    let proxy_service = GeminiProxyService::new(pool.as_ref().clone());
    let error_logger = ErrorLoggerService::new(pool.as_ref().clone());
    
    match proxy_service.forward_request("GET", "/v1/models", serde_json::json!({})).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            let error_msg = format!("Failed to list models: {}", e);
            if let Err(log_err) = error_logger.log_handler_error(
                None,
                "GET",
                "/v1/models",
                &error_msg,
                500,
                Some(start_time),
                None,
            ).await {
                tracing::warn!("Failed to log handler error: {}", log_err);
            }
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_model(
    Path(model): Path<String>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<Value>, StatusCode> {
    let start_time = Instant::now();
    let proxy_service = GeminiProxyService::new(pool.as_ref().clone());
    let error_logger = ErrorLoggerService::new(pool.as_ref().clone());
    let path = format!("/v1beta/models/{}", model);
    
    match proxy_service.forward_request("GET", &path, serde_json::json!({})).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            let error_msg = format!("Failed to get model {}: {}", model, e);
            if let Err(log_err) = error_logger.log_handler_error(
                None,
                "GET",
                &path,
                &error_msg,
                500,
                Some(start_time),
                None,
            ).await {
                tracing::warn!("Failed to log handler error: {}", log_err);
            }
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn get_model_by_path(
    Path(path): Path<String>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<Value>, StatusCode> {
    let start_time = Instant::now();
    let error_logger = ErrorLoggerService::new(pool.as_ref().clone());
    let full_path = format!("/v1beta/models/{}", path);
    
    // 只处理单个模型名的路径，不处理带有 : 的路径
    if path.contains(':') {
        let error_msg = format!("Invalid path format: {}", path);
        if let Err(log_err) = error_logger.log_handler_error(
            None,
            "GET",
            &full_path,
            &error_msg,
            404,
            Some(start_time),
            None,
        ).await {
            tracing::warn!("Failed to log handler error: {}", log_err);
        }
        return Err(StatusCode::NOT_FOUND);
    }
    
    let proxy_service = GeminiProxyService::new(pool.as_ref().clone());
    
    match proxy_service.forward_request("GET", &full_path, serde_json::json!({})).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            let error_msg = format!("Failed to get model {}: {}", path, e);
            if let Err(log_err) = error_logger.log_handler_error(
                None,
                "GET",
                &full_path,
                &error_msg,
                500,
                Some(start_time),
                None,
            ).await {
                tracing::warn!("Failed to log handler error: {}", log_err);
            }
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

pub async fn generate_content(
    Path(path): Path<String>,
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<Value>,
) -> Result<Response, StatusCode> {
    let start_time = Instant::now();
    let proxy_service = GeminiProxyService::new(pool.as_ref().clone());
    let error_logger = ErrorLoggerService::new(pool.as_ref().clone());
    let request_body = ErrorLoggerService::extract_request_body_string(&payload);
    
    // 处理 generateContent 和 streamGenerateContent 路径
    if path.ends_with(":generateContent") {
        let full_path = format!("/v1beta/models/{}", path);
        match proxy_service.forward_request("POST", &full_path, payload).await {
            Ok(response) => Ok(Json(response).into_response()),
            Err(e) => {
                let error_msg = e.to_string();
                
                // Return 400 for validation errors, 500 for other errors
                if error_msg.contains("Invalid request format") {
                    let error_response = serde_json::json!({
                        "error": {
                            "code": "INVALID_ARGUMENT",
                            "message": error_msg,
                            "status": "INVALID_ARGUMENT"
                        }
                    });
                    
                    if let Err(log_err) = error_logger.log_handler_error(
                        None,
                        "POST",
                        &full_path,
                        &error_msg,
                        400,
                        Some(start_time),
                        Some(&request_body),
                    ).await {
                        tracing::warn!("Failed to log handler error: {}", log_err);
                    }
                    
                    Ok((StatusCode::BAD_REQUEST, Json(error_response)).into_response())
                } else {
                    if let Err(log_err) = error_logger.log_handler_error(
                        None,
                        "POST",
                        &full_path,
                        &error_msg,
                        500,
                        Some(start_time),
                        Some(&request_body),
                    ).await {
                        tracing::warn!("Failed to log handler error: {}", log_err);
                    }
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
    } else if path.ends_with(":streamGenerateContent") {
        let full_path = format!("/v1beta/models/{}", path);
        match proxy_service.forward_streaming_request("POST", &full_path, payload).await {
            Ok(stream) => {
                let sse_stream = stream.map(|chunk| -> Result<Event, axum::Error> {
                    match chunk {
                        Ok(bytes) => {
                            let text = String::from_utf8_lossy(&bytes);
                            let lines: Vec<&str> = text.lines().collect();
                            for line in lines {
                                if line.starts_with("data: ") {
                                    let data = &line[6..];
                                    if !data.is_empty() && data != "[DONE]" {
                                        return Ok(Event::default().data(data.to_string()));
                                    }
                                }
                            }
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
                let error_msg = e.to_string();
                
                // Return 400 for validation errors, 500 for other errors
                if error_msg.contains("Invalid request format") {
                    if let Err(log_err) = error_logger.log_handler_error(
                        None,
                        "POST",
                        &full_path,
                        &error_msg,
                        400,
                        Some(start_time),
                        Some(&request_body),
                    ).await {
                        tracing::warn!("Failed to log handler error: {}", log_err);
                    }
                    Err(StatusCode::BAD_REQUEST)
                } else {
                    if let Err(log_err) = error_logger.log_handler_error(
                        None,
                        "POST",
                        &full_path,
                        &error_msg,
                        500,
                        Some(start_time),
                        Some(&request_body),
                    ).await {
                        tracing::warn!("Failed to log handler error: {}", log_err);
                    }
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
    } else {
        let error_msg = format!("Invalid endpoint path: {}", path);
        if let Err(log_err) = error_logger.log_handler_error(
            None,
            "POST",
            &path,
            &error_msg,
            404,
            Some(start_time),
            Some(&request_body),
        ).await {
            tracing::warn!("Failed to log handler error: {}", log_err);
        }
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn generate_content_v1(
    Path(path): Path<String>,
    State(pool): State<Arc<SqlitePool>>,
    Json(payload): Json<Value>,
) -> Result<Response, StatusCode> {
    let start_time = Instant::now();
    let proxy_service = GeminiProxyService::new(pool.as_ref().clone());
    let error_logger = ErrorLoggerService::new(pool.as_ref().clone());
    let request_body = ErrorLoggerService::extract_request_body_string(&payload);
    
    // 处理 generateContent 和 streamGenerateContent 路径，但使用 v1beta 转发
    if path.ends_with(":generateContent") {
        let full_path = format!("/v1beta/models/{}", path);
        match proxy_service.forward_request("POST", &full_path, payload).await {
            Ok(response) => Ok(Json(response).into_response()),
            Err(e) => {
                let error_msg = e.to_string();
                
                // Return 400 for validation errors, 500 for other errors
                if error_msg.contains("Invalid request format") {
                    let error_response = serde_json::json!({
                        "error": {
                            "code": "INVALID_ARGUMENT",
                            "message": error_msg,
                            "status": "INVALID_ARGUMENT"
                        }
                    });
                    
                    if let Err(log_err) = error_logger.log_handler_error(
                        None,
                        "POST",
                        &full_path,
                        &error_msg,
                        400,
                        Some(start_time),
                        Some(&request_body),
                    ).await {
                        tracing::warn!("Failed to log handler error: {}", log_err);
                    }
                    
                    Ok((StatusCode::BAD_REQUEST, Json(error_response)).into_response())
                } else {
                    if let Err(log_err) = error_logger.log_handler_error(
                        None,
                        "POST",
                        &full_path,
                        &error_msg,
                        500,
                        Some(start_time),
                        Some(&request_body),
                    ).await {
                        tracing::warn!("Failed to log handler error: {}", log_err);
                    }
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
    } else if path.ends_with(":streamGenerateContent") {
        let full_path = format!("/v1beta/models/{}", path);
        match proxy_service.forward_streaming_request("POST", &full_path, payload).await {
            Ok(stream) => {
                let sse_stream = stream.map(|chunk| -> Result<Event, axum::Error> {
                    match chunk {
                        Ok(bytes) => {
                            let text = String::from_utf8_lossy(&bytes);
                            let lines: Vec<&str> = text.lines().collect();
                            for line in lines {
                                if line.starts_with("data: ") {
                                    let data = &line[6..];
                                    if !data.is_empty() && data != "[DONE]" {
                                        return Ok(Event::default().data(data.to_string()));
                                    }
                                }
                            }
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
                let error_msg = e.to_string();
                
                // Return 400 for validation errors, 500 for other errors
                if error_msg.contains("Invalid request format") {
                    if let Err(log_err) = error_logger.log_handler_error(
                        None,
                        "POST",
                        &full_path,
                        &error_msg,
                        400,
                        Some(start_time),
                        Some(&request_body),
                    ).await {
                        tracing::warn!("Failed to log handler error: {}", log_err);
                    }
                    Err(StatusCode::BAD_REQUEST)
                } else {
                    if let Err(log_err) = error_logger.log_handler_error(
                        None,
                        "POST",
                        &full_path,
                        &error_msg,
                        500,
                        Some(start_time),
                        Some(&request_body),
                    ).await {
                        tracing::warn!("Failed to log handler error: {}", log_err);
                    }
                    Err(StatusCode::INTERNAL_SERVER_ERROR)
                }
            }
        }
    } else {
        let error_msg = format!("Invalid endpoint path: {}", path);
        if let Err(log_err) = error_logger.log_handler_error(
            None,
            "POST",
            &path,
            &error_msg,
            404,
            Some(start_time),
            Some(&request_body),
        ).await {
            tracing::warn!("Failed to log handler error: {}", log_err);
        }
        Err(StatusCode::NOT_FOUND)
    }
}

pub async fn get_model_by_path_v1(
    Path(path): Path<String>,
    State(pool): State<Arc<SqlitePool>>,
) -> Result<Json<Value>, StatusCode> {
    let start_time = Instant::now();
    let error_logger = ErrorLoggerService::new(pool.as_ref().clone());
    let full_path = format!("/v1beta/models/{}", path);
    
    // 只处理单个模型名的路径，不处理带有 : 的路径
    if path.contains(':') {
        let error_msg = format!("Invalid path format: {}", path);
        if let Err(log_err) = error_logger.log_handler_error(
            None,
            "GET",
            &full_path,
            &error_msg,
            404,
            Some(start_time),
            None,
        ).await {
            tracing::warn!("Failed to log handler error: {}", log_err);
        }
        return Err(StatusCode::NOT_FOUND);
    }
    
    let proxy_service = GeminiProxyService::new(pool.as_ref().clone());
    
    match proxy_service.forward_request("GET", &full_path, serde_json::json!({})).await {
        Ok(response) => Ok(Json(response)),
        Err(e) => {
            let error_msg = format!("Failed to get model {}: {}", path, e);
            if let Err(log_err) = error_logger.log_handler_error(
                None,
                "GET",
                &full_path,
                &error_msg,
                500,
                Some(start_time),
                None,
            ).await {
                tracing::warn!("Failed to log handler error: {}", log_err);
            }
            Err(StatusCode::INTERNAL_SERVER_ERROR)
        }
    }
}

