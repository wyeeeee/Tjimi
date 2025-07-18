use crate::models::RequestLogResponse;
use serde::Serialize;
use tauri::State;
use sqlx::SqlitePool;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct LogResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedLogsResponse {
    pub logs: Vec<RequestLogResponse>,
    pub total_count: u32,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

#[tauri::command]
pub async fn get_request_logs(
    limit: Option<i32>,
    pool: State<'_, SqlitePool>,
) -> Result<LogResult<Vec<RequestLogResponse>>, String> {
    let limit = limit.unwrap_or(100);
    
    let logs: Result<Vec<RequestLogResponse>, sqlx::Error> = sqlx::query_as(
        r#"
        SELECT 
            rl.id,
            ak.name as api_key_name,
            rl.method,
            rl.path,
            rl.status_code,
            rl.response_time_ms,
            rl.created_at
        FROM request_logs rl
        JOIN api_keys ak ON rl.api_key_id = ak.id
        ORDER BY rl.created_at DESC
        LIMIT ?
        "#,
    )
    .bind(limit)
    .fetch_all(pool.inner())
    .await;
    
    match logs {
        Ok(logs) => Ok(LogResult {
            success: true,
            data: Some(logs),
            error: None,
        }),
        Err(e) => Ok(LogResult {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn get_usage_stats(
    pool: State<'_, SqlitePool>,
) -> Result<LogResult<serde_json::Value>, String> {
    let stats: Result<(i64, i64, Option<f64>), sqlx::Error> = sqlx::query_as(
        r#"
        SELECT 
            COUNT(*) as total_requests,
            SUM(ak.usage_count) as total_usage,
            AVG(rl.response_time_ms) as avg_response_time
        FROM request_logs rl
        JOIN api_keys ak ON rl.api_key_id = ak.id
        "#
    )
    .fetch_one(pool.inner())
    .await;
    
    match stats {
        Ok((total_requests, total_usage, avg_response_time)) => {
            let stats = serde_json::json!({
                "totalRequests": total_requests,
                "totalUsage": total_usage,
                "avgResponseTime": avg_response_time.unwrap_or(0.0)
            });
            
            Ok(LogResult {
                success: true,
                data: Some(stats),
                error: None,
            })
        },
        Err(e) => Ok(LogResult {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn get_request_logs_paginated(
    page: Option<u32>,
    per_page: Option<u32>,
    pool: State<'_, SqlitePool>,
) -> Result<LogResult<PaginatedLogsResponse>, String> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(50).min(200); // 最多200条
    let offset = (page - 1) * per_page;
    
    // 获取总数
    let total_count_result: Result<(i64,), sqlx::Error> = sqlx::query_as(
        "SELECT COUNT(*) FROM request_logs"
    )
    .fetch_one(pool.inner())
    .await;
    
    let total_count = match total_count_result {
        Ok((count,)) => count as u32,
        Err(e) => return Ok(LogResult {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    };
    
    // 获取分页数据
    let logs: Result<Vec<RequestLogResponse>, sqlx::Error> = sqlx::query_as(
        r#"
        SELECT 
            rl.id,
            ak.name as api_key_name,
            rl.method,
            rl.path,
            rl.status_code,
            rl.response_time_ms,
            rl.created_at
        FROM request_logs rl
        JOIN api_keys ak ON rl.api_key_id = ak.id
        ORDER BY rl.created_at DESC
        LIMIT ? OFFSET ?
        "#,
    )
    .bind(per_page)
    .bind(offset)
    .fetch_all(pool.inner())
    .await;
    
    match logs {
        Ok(logs) => {
            let total_pages = (total_count + per_page - 1) / per_page;
            
            Ok(LogResult {
                success: true,
                data: Some(PaginatedLogsResponse {
                    logs,
                    total_count,
                    page,
                    per_page,
                    total_pages,
                }),
                error: None,
            })
        },
        Err(e) => Ok(LogResult {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}