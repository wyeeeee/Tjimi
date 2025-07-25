use crate::services::{KeyRotationService, ApiKeyService, SettingsService};
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
    key_rotation: KeyRotationService,
    api_key_service: ApiKeyService,
    settings_service: SettingsService,
    pool: SqlitePool,
}

impl GeminiProxyService {
    pub fn new(pool: SqlitePool) -> Self {
        let key_rotation = KeyRotationService::new(pool.clone());
        let api_key_service = ApiKeyService::new(pool.clone());
        let settings_service = SettingsService::new(pool.clone());

        Self {
            key_rotation,
            api_key_service,
            settings_service,
            pool,
        }
    }

    // 动态创建HTTP客户端，根据当前代理设置
    async fn create_client(&self) -> Result<Client> {
        let proxy_settings = self.settings_service.get_proxy_settings().await?;
        
        // 添加调试日志
        tracing::info!("代理设置状态: enabled={}, type={}, host={:?}, port={:?}", 
                      proxy_settings.enabled, 
                      proxy_settings.proxy_type, 
                      proxy_settings.host, 
                      proxy_settings.port);
        
        let mut client_builder = Client::builder()
            .timeout(std::time::Duration::from_secs(60));

        // 如果启用了代理，则配置代理
        if proxy_settings.enabled {
            if let (Some(host), Some(port)) = (&proxy_settings.host, proxy_settings.port) {
                let proxy_url = format!("{}://{}:{}", proxy_settings.proxy_type, host, port);
                tracing::info!("应用代理配置: {}", proxy_url);
                
                let proxy = match proxy_settings.proxy_type.as_str() {
                    "http" => reqwest::Proxy::http(&proxy_url)?,
                    "https" => reqwest::Proxy::https(&proxy_url)?,
                    "socks4" | "socks5" => reqwest::Proxy::all(&proxy_url)?,
                    _ => reqwest::Proxy::http(&proxy_url)?,
                };

                // 如果有认证信息，添加认证
                let proxy = if let (Some(username), Some(password)) = (&proxy_settings.username, &proxy_settings.password) {
                    proxy.basic_auth(username, password)
                } else {
                    proxy
                };

                client_builder = client_builder.proxy(proxy);
            } else {
                tracing::warn!("代理已启用但主机或端口为空");
            }
        } else {
            tracing::info!("代理已禁用，使用直连");
        }

        client_builder.build()
            .map_err(|e| anyhow!("Failed to create HTTP client: {}", e))
    }

    pub async fn forward_request(&self, method: &str, path: &str, body: Value) -> Result<Value> {
        // Validate request body for generateContent endpoints
        if method == "POST" && (path.contains(":generateContent") || path.contains("generateContent")) {
            if let Err(e) = self.validate_generate_content_request(&body) {
                return Err(anyhow!("Invalid request format: {}", e));
            }
        }

        let retry_count = self.settings_service.get_retry_count().await.unwrap_or(3);
        
        for attempt in 0..retry_count {
            let start_time = Instant::now();
            
            let api_key = self.key_rotation.get_next_active_key().await?
                .ok_or_else(|| anyhow!("No active API keys available"))?;

            // Convert v1 paths to v1beta and add API key as query parameter
            let converted_path = path.replace("/v1/", "/v1beta/");
            let gemini_url = format!("https://generativelanguage.googleapis.com{}?key={}", converted_path, api_key.key_value);
            
            // 创建一次客户端，应用当前代理设置
            let client = self.create_client().await?;
            
            let mut request = match method {
                "GET" => client.get(&gemini_url),
                "POST" => client.post(&gemini_url),
                "PUT" => client.put(&gemini_url),
                "DELETE" => client.delete(&gemini_url),
                _ => return Err(anyhow!("Unsupported HTTP method: {}", method)),
            };

            request = request.header("Content-Type", "application/json");

            if method != "GET" {
                request = request.json(&body);
            }

            let response = request.send().await?;
            let status_code = response.status().as_u16() as i32;
            let response_time = start_time.elapsed().as_millis() as i64;

            // Initial log (will be updated later with body content)

            // Update API key usage
            self.api_key_service.increment_usage(api_key.id).await?;

            if response.status().is_success() {
                let response_text = response.text().await?;
                let json_response: Value = serde_json::from_str(&response_text)?;
                
                // Update log with request and response body
                let request_body_str = if method != "GET" { Some(body.to_string()) } else { None };
                if let Err(e) = self.log_request_with_body(
                    api_key.id, 
                    method, 
                    path, 
                    status_code, 
                    response_time, 
                    request_body_str.as_deref(), 
                    Some(&response_text)
                ).await {
                    tracing::warn!("Failed to update log with body: {}", e);
                }
                
                return Ok(json_response);
            } else {
                // If the API key is invalid, mark it as failed
                if status_code == 401 || status_code == 403 {
                    self.key_rotation.mark_key_as_failed(api_key.id).await?;
                }
                
                let error_text = response.text().await?;
                
                // Update log with request and error response body
                let request_body_str = if method != "GET" { Some(body.to_string()) } else { None };
                if let Err(e) = self.log_request_with_body(
                    api_key.id, 
                    method, 
                    path, 
                    status_code, 
                    response_time, 
                    request_body_str.as_deref(), 
                    Some(&error_text)
                ).await {
                    tracing::warn!("Failed to update log with body: {}", e);
                }
                
                // If this is the last attempt, return the error
                if attempt == retry_count - 1 {
                    return Err(anyhow!("Gemini API error after {} attempts ({}): {}", retry_count, status_code, error_text));
                }
                
                // Log retry attempt
                tracing::warn!("Request failed (attempt {}/{}): {} - {}", attempt + 1, retry_count, status_code, error_text);
                
                // Wait a bit before retrying (exponential backoff)
                let delay = std::time::Duration::from_millis(100 * (2_u64.pow(attempt as u32)));
                tokio::time::sleep(delay).await;
            }
        }
        
        Err(anyhow!("Request failed after all retry attempts"))
    }

    pub async fn forward_streaming_request(&self, method: &str, path: &str, body: Value) -> Result<impl tokio_stream::Stream<Item = Result<Bytes>> + use<>> {
        // Validate request body for streaming generateContent endpoints
        if method == "POST" && (path.contains(":streamGenerateContent") || path.contains("streamGenerateContent")) {
            if let Err(e) = self.validate_generate_content_request(&body) {
                return Err(anyhow!("Invalid request format: {}", e));
            }
        }

        let retry_count = self.settings_service.get_retry_count().await.unwrap_or(3);
        
        for attempt in 0..retry_count {
            let start_time = Instant::now();
            let api_key = self.key_rotation.get_next_active_key().await?
                .ok_or_else(|| anyhow!("No active API keys available"))?;

            // Convert v1 paths to v1beta, add API key and alt=sse for streaming
            let converted_path = path.replace("/v1/", "/v1beta/");
            let gemini_url = format!("https://generativelanguage.googleapis.com{}?key={}&alt=sse", converted_path, api_key.key_value);
            
            tracing::info!("Starting streaming request (attempt {}/{}): {}", attempt + 1, retry_count, &gemini_url[..100]);
            
            // 创建一次客户端，应用当前代理设置
            let client = self.create_client().await?;
            
            let mut request = match method {
                "GET" => client.get(&gemini_url),
                "POST" => client.post(&gemini_url),
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
            
            if response.status().is_success() {
                // Update API key usage
                if let Err(e) = self.api_key_service.increment_usage(api_key.id).await {
                    tracing::warn!("Failed to increment API key usage: {}", e);
                }

                // Log successful streaming start with request body
                let response_time = start_time.elapsed().as_millis() as i64;
                let request_body_str = if method != "GET" { Some(body.to_string()) } else { None };
                if let Err(e) = self.log_request_with_body(
                    api_key.id, 
                    method, 
                    path, 
                    status_code, 
                    response_time, 
                    request_body_str.as_deref(), 
                    Some("[Streaming Response]")
                ).await {
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

                return Ok(stream);
            } else {
                let response_time = start_time.elapsed().as_millis() as i64;
                
                let error_text = response.text().await?;
                
                // Log the failed request with bodies
                let request_body_str = if method != "GET" { Some(body.to_string()) } else { None };
                if let Err(e) = self.log_request_with_body(
                    api_key.id, 
                    method, 
                    path, 
                    status_code, 
                    response_time, 
                    request_body_str.as_deref(), 
                    Some(&error_text)
                ).await {
                    tracing::warn!("Failed to log streaming request: {}", e);
                }
                
                // If the API key is invalid, mark it as failed
                if status_code == 401 || status_code == 403 {
                    if let Err(e) = self.key_rotation.mark_key_as_failed(api_key.id).await {
                        tracing::warn!("Failed to mark key as failed: {}", e);
                    }
                }
                
                // If this is the last attempt, return the error
                if attempt == retry_count - 1 {
                    return Err(anyhow!("Streaming request failed after {} attempts ({}): {}", retry_count, status_code, error_text));
                }
                
                // Log retry attempt
                tracing::warn!("Streaming request failed (attempt {}/{}): {} - {}", attempt + 1, retry_count, status_code, error_text);
                
                // Wait a bit before retrying (exponential backoff)
                let delay = std::time::Duration::from_millis(100 * (2_u64.pow(attempt as u32)));
                tokio::time::sleep(delay).await;
            }
        }
        
        Err(anyhow!("Streaming request failed after all retry attempts"))
    }

    async fn log_request(&self, api_key_id: Uuid, method: &str, path: &str, status_code: i32, response_time_ms: i64) -> Result<()> {
        self.log_request_with_body(api_key_id, method, path, status_code, response_time_ms, None, None).await
    }

    async fn log_request_with_body(&self, api_key_id: Uuid, method: &str, path: &str, status_code: i32, response_time_ms: i64, request_body: Option<&str>, response_body: Option<&str>) -> Result<()> {
        let log_id = Uuid::new_v4();
        let now = Utc::now();

        sqlx::query(
            r#"
            INSERT INTO request_logs (id, api_key_id, method, path, status_code, response_time_ms, request_body, response_body, created_at)
            VALUES (?, ?, ?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(log_id.to_string())
        .bind(api_key_id.to_string())
        .bind(method)
        .bind(path)
        .bind(status_code)
        .bind(response_time_ms)
        .bind(request_body)
        .bind(response_body)
        .bind(to_js_compatible_timestamp(now))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    fn validate_generate_content_request(&self, body: &Value) -> Result<()> {
        // Check if body is an object
        let obj = body.as_object()
            .ok_or_else(|| anyhow!("Request body must be a JSON object"))?;

        // Check if contents field exists and is not null
        let contents = obj.get("contents")
            .ok_or_else(|| anyhow!("Missing required field 'contents'"))?;

        // Check if contents is an array
        let contents_array = contents.as_array()
            .ok_or_else(|| anyhow!("Field 'contents' must be an array"))?;

        // Check if contents array is not empty
        if contents_array.is_empty() {
            return Err(anyhow!("Field 'contents' cannot be empty"));
        }

        // Validate each content item has parts
        for (index, content) in contents_array.iter().enumerate() {
            let content_obj = content.as_object()
                .ok_or_else(|| anyhow!("Content item {} must be an object", index))?;
            
            let parts = content_obj.get("parts")
                .ok_or_else(|| anyhow!("Content item {} missing required field 'parts'", index))?;
            
            let parts_array = parts.as_array()
                .ok_or_else(|| anyhow!("Field 'parts' in content item {} must be an array", index))?;
            
            if parts_array.is_empty() {
                return Err(anyhow!("Field 'parts' in content item {} cannot be empty", index));
            }

            // Validate each part has at least one content field (text, inline_data, etc.)
            for (part_index, part) in parts_array.iter().enumerate() {
                let part_obj = part.as_object()
                    .ok_or_else(|| anyhow!("Part {} in content item {} must be an object", part_index, index))?;
                
                let has_content = part_obj.contains_key("text") || 
                                 part_obj.contains_key("inline_data") ||
                                 part_obj.contains_key("function_call") ||
                                 part_obj.contains_key("function_response");
                
                if !has_content {
                    return Err(anyhow!("Part {} in content item {} must contain at least one content field (text, inline_data, function_call, or function_response)", part_index, index));
                }
            }
        }

        Ok(())
    }
}