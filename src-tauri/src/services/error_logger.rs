use anyhow::Result;
use chrono::{Utc, SecondsFormat};
use serde_json::Value;
use sqlx::SqlitePool;
use std::time::Instant;
use uuid::Uuid;

fn to_js_compatible_timestamp(dt: chrono::DateTime<Utc>) -> String {
    dt.to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub struct ErrorLoggerService {
    pool: SqlitePool,
}

impl ErrorLoggerService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    /// 记录认证或中间件错误
    pub async fn log_auth_error(
        &self,
        method: &str,
        path: &str,
        error_msg: &str,
        status_code: i32,
        request_body: Option<&str>,
    ) -> Result<()> {
        let log_id = Uuid::new_v4();
        let now = Utc::now();
        
        let error_response = serde_json::json!({
            "error": {
                "code": if status_code == 401 { "UNAUTHENTICATED" } else if status_code == 403 { "PERMISSION_DENIED" } else { "INTERNAL_ERROR" },
                "message": error_msg,
                "status": if status_code == 401 { "UNAUTHENTICATED" } else if status_code == 403 { "PERMISSION_DENIED" } else { "INTERNAL" }
            }
        });

        sqlx::query(
            r#"
            INSERT INTO request_logs (id, api_key_id, method, path, status_code, response_time_ms, request_body, response_body, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(log_id.to_string())
        .bind("") // 认证失败时没有有效的API key ID
        .bind(method)
        .bind(path)
        .bind(status_code)
        .bind(0i64) // 认证错误响应时间很短
        .bind(request_body)
        .bind(error_response.to_string())
        .bind(to_js_compatible_timestamp(now))
        .execute(&self.pool)
        .await?;

        tracing::error!("Auth/Middleware error - Method: {}, Path: {}, Status: {}, Error: {}", 
                       method, path, status_code, error_msg);

        Ok(())
    }

    /// 记录处理器级别的错误
    pub async fn log_handler_error(
        &self,
        api_key_id: Option<Uuid>,
        method: &str,
        path: &str,
        error_msg: &str,
        status_code: i32,
        response_time: Option<Instant>,
        request_body: Option<&str>,
    ) -> Result<()> {
        let log_id = Uuid::new_v4();
        let now = Utc::now();
        let response_time_ms = response_time.map(|t| t.elapsed().as_millis() as i64).unwrap_or(0);
        
        let error_response = serde_json::json!({
            "error": {
                "code": match status_code {
                    400 => "INVALID_ARGUMENT",
                    401 => "UNAUTHENTICATED", 
                    403 => "PERMISSION_DENIED",
                    404 => "NOT_FOUND",
                    500 => "INTERNAL",
                    _ => "UNKNOWN"
                },
                "message": error_msg,
                "status": match status_code {
                    400 => "INVALID_ARGUMENT",
                    401 => "UNAUTHENTICATED",
                    403 => "PERMISSION_DENIED", 
                    404 => "NOT_FOUND",
                    500 => "INTERNAL",
                    _ => "UNKNOWN"
                }
            }
        });

        sqlx::query(
            r#"
            INSERT INTO request_logs (id, api_key_id, method, path, status_code, response_time_ms, request_body, response_body, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(log_id.to_string())
        .bind(api_key_id.map(|id| id.to_string()).unwrap_or_default())
        .bind(method)
        .bind(path)
        .bind(status_code)
        .bind(response_time_ms)
        .bind(request_body)
        .bind(error_response.to_string())
        .bind(to_js_compatible_timestamp(now))
        .execute(&self.pool)
        .await?;

        tracing::error!("Handler error - Method: {}, Path: {}, Status: {}, Error: {}, Response time: {}ms", 
                       method, path, status_code, error_msg, response_time_ms);

        Ok(())
    }

    /// 提取请求体内容（如果可能的话）
    pub fn extract_request_body_string(body: &Value) -> String {
        serde_json::to_string_pretty(body).unwrap_or_else(|_| "Invalid JSON".to_string())
    }
}