use axum::{
    extract::{Query, State},
    http::{Request, StatusCode},
    middleware::Next,
    response::Response,
};
use std::sync::Arc;
use std::collections::HashMap;
use crate::services::CustomAuthService;
use sqlx::SqlitePool;

pub async fn custom_auth_middleware(
    State(pool): State<Arc<SqlitePool>>,
    req: Request<axum::body::Body>,
    next: Next,
) -> Result<Response, StatusCode> {
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
            tracing::warn!("No authorization provided (neither header nor query param)");
            return Err(StatusCode::UNAUTHORIZED);
        }
    };

    // 验证自定义密钥
    let custom_auth_service = CustomAuthService::new(pool.as_ref().clone());
    match custom_auth_service.validate_custom_key(&key_value).await {
        Ok(is_valid) => {
            if !is_valid {
                tracing::warn!("Invalid custom key provided: {}", mask_key(&key_value));
                return Err(StatusCode::FORBIDDEN);
            }
        }
        Err(e) => {
            tracing::error!("Failed to validate custom key: {}", e);
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