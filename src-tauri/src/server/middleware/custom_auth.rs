use axum::{
    extract::State,
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
    body::to_bytes,
};
use std::sync::Arc;
use std::collections::HashMap;
use crate::services::{CustomAuthService, ErrorLoggerService};
use sqlx::SqlitePool;

pub async fn custom_auth_middleware(
    State(pool): State<Arc<SqlitePool>>,
    mut req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
    let error_logger = ErrorLoggerService::new(pool.as_ref().clone());
    let method = req.method().to_string();
    let path = req.uri().path().to_string();
    
    // 提取请求体用于日志记录（如果是POST请求）
    let request_body = if req.method() == "POST" {
        let (parts, body) = req.into_parts();
        match to_bytes(body, usize::MAX).await {
            Ok(bytes) => {
                let body_str = String::from_utf8_lossy(&bytes).to_string();
                // 重新构建请求
                req = Request::from_parts(parts, axum::body::Body::from(bytes.clone()));
                Some(body_str)
            }
            Err(_) => {
                req = Request::from_parts(parts, axum::body::Body::empty());
                None
            }
        }
    } else {
        None
    };

    // 自定义验证现在是强制性的，所有API请求都必须提供有效的自定义密钥
    // 从请求头获取自定义验证密钥
    let auth_header_key = if let Some(auth_header) = req.headers().get("authorization") {
        if let Ok(auth_str) = auth_header.to_str() {
            if auth_str.starts_with("Bearer ") {
                Some(auth_str[7..].to_string())
            } else {
                None
            }
        } else {
            None
        }
    } else {
        None
    };

    // 从query参数获取key (用于SillyTavern兼容性)
    let query_key = if let Some(query) = req.uri().query() {
        let params: HashMap<String, String> = url::form_urlencoded::parse(query.as_bytes())
            .into_owned()
            .collect();
        params.get("key").cloned()
    } else {
        None
    };

    // 如果没有提供自定义密钥，返回未授权
    let key_value = match auth_header_key.or(query_key) {
        Some(key) => key,
        None => {
            let error_msg = "No authorization provided (neither header nor query param)";
            if let Err(e) = error_logger.log_auth_error(
                &method,
                &path,
                error_msg,
                401,
                request_body.as_deref(),
            ).await {
                tracing::warn!("Failed to log auth error: {}", e);
            }
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // 验证自定义密钥
    let custom_auth_service = CustomAuthService::new(pool.as_ref().clone());
    match custom_auth_service.validate_custom_key(&key_value).await {
        Ok(is_valid) => {
            if !is_valid {
                let error_msg = format!("Invalid custom key provided: {}", mask_key(&key_value));
                if let Err(e) = error_logger.log_auth_error(
                    &method,
                    &path,
                    &error_msg,
                    403,
                    request_body.as_deref(),
                ).await {
                    tracing::warn!("Failed to log auth error: {}", e);
                }
                return Err(StatusCode::FORBIDDEN);
            }
        }
        Err(e) => {
            let error_msg = format!("Failed to validate custom key: {}", e);
            if let Err(log_err) = error_logger.log_auth_error(
                &method,
                &path,
                &error_msg,
                500,
                request_body.as_deref(),
            ).await {
                tracing::warn!("Failed to log auth error: {}", log_err);
            }
            return Err(StatusCode::INTERNAL_SERVER_ERROR);
        }
    }

    // 如果验证通过，继续处理请求
    Ok(next.run(req).await)
}

fn mask_key(key: &str) -> String {
    if key.len() > 8 {
        format!("{}****{}", &key[..4], &key[key.len()-4..])
    } else {
        "****".to_string()
    }
}