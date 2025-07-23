use crate::models::RequestLogResponse;
use serde::Serialize;
use tauri::State;
use sqlx::SqlitePool;
use chrono::{Timelike, TimeZone};

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
            rl.request_body,
            rl.response_body,
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
pub async fn get_api_key_today_requests(
    api_key_id: String,
    pool: State<'_, SqlitePool>,
) -> Result<LogResult<i64>, String> {
    // Calculate today's reset time (3 PM UTC+8 = 7 AM UTC)
    let now = chrono::Utc::now();
    let china_tz = chrono::FixedOffset::east_opt(8 * 3600).unwrap(); // UTC+8
    let now_china = now.with_timezone(&china_tz);
    
    // Get today's reset time (3 PM China time = 7 AM UTC)
    let today_reset = if now_china.hour() >= 15 {
        // If it's after 3 PM, today's reset already happened
        now_china.date_naive().and_hms_opt(15, 0, 0).unwrap()
    } else {
        // If it's before 3 PM, use yesterday's reset
        (now_china.date_naive() - chrono::Duration::days(1)).and_hms_opt(15, 0, 0).unwrap()
    };
    
    let reset_time_utc = china_tz.from_local_datetime(&today_reset).unwrap().with_timezone(&chrono::Utc);
    let reset_time_str = reset_time_utc.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    
    // Get today's requests for this API key
    let today_requests: Result<(i64,), sqlx::Error> = sqlx::query_as(
        r#"
        SELECT COUNT(*) as today_requests
        FROM request_logs rl
        WHERE rl.api_key_id = ? AND rl.created_at >= ?
        "#
    )
    .bind(&api_key_id)
    .bind(&reset_time_str)
    .fetch_one(pool.inner())
    .await;
    
    match today_requests {
        Ok((count,)) => Ok(LogResult {
            success: true,
            data: Some(count),
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
    // Calculate today's reset time (3 PM UTC+8 = 7 AM UTC)
    let now = chrono::Utc::now();
    let china_tz = chrono::FixedOffset::east_opt(8 * 3600).unwrap(); // UTC+8
    let now_china = now.with_timezone(&china_tz);
    
    // Get today's reset time (3 PM China time = 7 AM UTC)
    let today_reset = if now_china.hour() >= 15 {
        // If it's after 3 PM, today's reset already happened
        now_china.date_naive().and_hms_opt(15, 0, 0).unwrap()
    } else {
        // If it's before 3 PM, use yesterday's reset
        (now_china.date_naive() - chrono::Duration::days(1)).and_hms_opt(15, 0, 0).unwrap()
    };
    
    let reset_time_utc = china_tz.from_local_datetime(&today_reset).unwrap().with_timezone(&chrono::Utc);
    let reset_time_str = reset_time_utc.to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
    
    // Get total stats
    let total_stats: Result<(i64, i64, Option<f64>), sqlx::Error> = sqlx::query_as(
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
    
    // Get today's stats (since the last 3 PM reset)
    let today_stats: Result<(i64, Option<f64>), sqlx::Error> = sqlx::query_as(
        r#"
        SELECT 
            COUNT(*) as today_requests,
            AVG(rl.response_time_ms) as today_avg_response_time
        FROM request_logs rl
        WHERE rl.created_at >= ?
        "#
    )
    .bind(&reset_time_str)
    .fetch_one(pool.inner())
    .await;
    
    match (total_stats, today_stats) {
        (Ok((total_requests, total_usage, avg_response_time)), Ok((today_requests, today_avg_response_time))) => {
            // Get today's requests per API key
            let api_key_today_requests: Result<Vec<(String, i64)>, sqlx::Error> = sqlx::query_as(
                r#"
                SELECT rl.api_key_id, COUNT(*) as today_requests
                FROM request_logs rl
                WHERE rl.created_at >= ?
                GROUP BY rl.api_key_id
                "#
            )
            .bind(&reset_time_str)
            .fetch_all(pool.inner())
            .await;

            let api_key_today_map: std::collections::HashMap<String, i64> = match api_key_today_requests {
                Ok(results) => results.into_iter().collect(),
                Err(_) => std::collections::HashMap::new(),
            };

            let stats = serde_json::json!({
                "totalRequests": total_requests,
                "totalUsage": total_usage,
                "avgResponseTime": avg_response_time.unwrap_or(0.0),
                "todayRequests": today_requests,
                "todayAvgResponseTime": today_avg_response_time.unwrap_or(0.0),
                "resetTime": reset_time_str,
                "apiKeyTodayRequests": api_key_today_map
            });
            
            Ok(LogResult {
                success: true,
                data: Some(stats),
                error: None,
            })
        },
        (Err(e), _) | (_, Err(e)) => Ok(LogResult {
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
            rl.request_body,
            rl.response_body,
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