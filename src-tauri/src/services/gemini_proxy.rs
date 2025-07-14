use crate::services::{KeyRotationService, ApiKeyService};
use anyhow::{Result, anyhow};
use reqwest::Client;
use serde_json::Value;
use std::time::Instant;
use tokio_stream::StreamExt;
use uuid::Uuid;
use chrono::{Utc, SecondsFormat};
use sqlx::SqlitePool;
use bytes::Bytes;

fn to_js_compatible_timestamp(dt: chrono::DateTime<Utc>) -> String {
    // 生成 JavaScript 兼容的时间戳（毫秒精度）
    dt.to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub struct GeminiProxyService {
    client: Client,
    key_rotation: KeyRotationService,
    api_key_service: ApiKeyService,
    pool: SqlitePool,
}

impl GeminiProxyService {
    pub fn new(pool: SqlitePool) -> Self {
        let client = Client::builder()
            .timeout(std::time::Duration::from_secs(60))
            .build()
            .expect("Failed to create HTTP client");

        let key_rotation = KeyRotationService::new(pool.clone());
        let api_key_service = ApiKeyService::new(pool.clone());

        Self {
            client,
            key_rotation,
            api_key_service,
            pool,
        }
    }

    pub async fn forward_request(&self, method: &str, path: &str, body: Value) -> Result<Value> {
        let start_time = Instant::now();
        
        let api_key = self.key_rotation.get_next_active_key().await?
            .ok_or_else(|| anyhow!("No active API keys available"))?;

        // Convert v1 paths to v1beta and add API key as query parameter
        let converted_path = path.replace("/v1/", "/v1beta/");
        let gemini_url = format!("https://generativelanguage.googleapis.com{}?key={}", converted_path, api_key.key_value);
        
        let mut request = match method {
            "GET" => self.client.get(&gemini_url),
            "POST" => self.client.post(&gemini_url),
            "PUT" => self.client.put(&gemini_url),
            "DELETE" => self.client.delete(&gemini_url),
            _ => return Err(anyhow!("Unsupported HTTP method: {}", method)),
        };

        request = request.header("Content-Type", "application/json");

        if method != "GET" {
            request = request.json(&body);
        }

        let response = request.send().await?;
        let status_code = response.status().as_u16() as i32;
        let response_time = start_time.elapsed().as_millis() as i64;

        // Log the request
        self.log_request(api_key.id, method, path, status_code, response_time).await?;

        // Update API key usage
        self.api_key_service.increment_usage(api_key.id).await?;

        if response.status().is_success() {
            let json_response: Value = response.json().await?;
            Ok(json_response)
        } else {
            // If the API key is invalid, mark it as failed
            if status_code == 401 || status_code == 403 {
                self.key_rotation.mark_key_as_failed(api_key.id).await?;
            }
            
            let error_text = response.text().await?;
            Err(anyhow!("Gemini API error ({}): {}", status_code, error_text))
        }
    }

    pub async fn forward_streaming_request(&self, method: &str, path: &str, body: Value) -> Result<impl tokio_stream::Stream<Item = Result<Bytes>>> {
        let start_time = Instant::now();
        let api_key = self.key_rotation.get_next_active_key().await?
            .ok_or_else(|| anyhow!("No active API keys available"))?;

        // Convert v1 paths to v1beta, add API key and alt=sse for streaming
        let converted_path = path.replace("/v1/", "/v1beta/");
        let gemini_url = format!("https://generativelanguage.googleapis.com{}?key={}&alt=sse", converted_path, api_key.key_value);
        
        tracing::info!("Starting streaming request to: {}", &gemini_url[..100]);
        
        let mut request = match method {
            "GET" => self.client.get(&gemini_url),
            "POST" => self.client.post(&gemini_url),
            _ => return Err(anyhow!("Unsupported HTTP method for streaming: {}", method)),
        };

        request = request
            .header("Content-Type", "application/json")
            .header("Accept", "text/event-stream")
            .header("Cache-Control", "no-cache");

        if method != "GET" {
            request = request.json(&body);
        }

        let response = request.send().await?;
        let status_code = response.status().as_u16() as i32;
        
        tracing::info!("Streaming response status: {}", status_code);
        
        if !response.status().is_success() {
            let response_time = start_time.elapsed().as_millis() as i64;
            
            // Log the failed request
            if let Err(e) = self.log_request(api_key.id, method, path, status_code, response_time).await {
                tracing::warn!("Failed to log streaming request: {}", e);
            }
            
            // If the API key is invalid, mark it as failed
            if status_code == 401 || status_code == 403 {
                if let Err(e) = self.key_rotation.mark_key_as_failed(api_key.id).await {
                    tracing::warn!("Failed to mark key as failed: {}", e);
                }
            }
            
            let error_text = response.text().await?;
            return Err(anyhow!("Gemini API error ({}): {}", status_code, error_text));
        }

        // Update API key usage
        if let Err(e) = self.api_key_service.increment_usage(api_key.id).await {
            tracing::warn!("Failed to increment API key usage: {}", e);
        }

        // Log successful streaming start
        let response_time = start_time.elapsed().as_millis() as i64;
        if let Err(e) = self.log_request(api_key.id, method, path, status_code, response_time).await {
            tracing::warn!("Failed to log streaming request: {}", e);
        }

        let stream = response.bytes_stream()
            .map(|result| {
                match result {
                    Ok(bytes) => {
                        tracing::debug!("Received {} bytes in stream", bytes.len());
                        Ok(bytes)
                    }
                    Err(e) => {
                        tracing::error!("Stream bytes error: {}", e);
                        Err(anyhow!("Stream error: {}", e))
                    }
                }
            });

        Ok(stream)
    }

    async fn log_request(&self, api_key_id: Uuid, method: &str, path: &str, status_code: i32, response_time_ms: i64) -> Result<()> {
        let log_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO request_logs (id, api_key_id, method, path, status_code, response_time_ms, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(log_id.to_string())
        .bind(api_key_id.to_string())
        .bind(method)
        .bind(path)
        .bind(status_code)
        .bind(response_time_ms)
        .bind(to_js_compatible_timestamp(now))
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}