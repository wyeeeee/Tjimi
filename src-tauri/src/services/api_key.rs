use crate::models::{ApiKey, CreateApiKeyRequest, UpdateApiKeyRequest, ApiKeyResponse};
use sqlx::SqlitePool;
use anyhow::Result;
use chrono::{Utc, SecondsFormat};
use uuid::Uuid;

fn to_js_compatible_timestamp(dt: chrono::DateTime<Utc>) -> String {
    // 生成 JavaScript 兼容的时间戳（毫秒精度）
    dt.to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub struct ApiKeyService {
    pool: SqlitePool,
}

impl ApiKeyService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn create_api_key(&self, request: CreateApiKeyRequest) -> Result<ApiKeyResponse> {
        let key_id = Uuid::new_v4();
        let now = Utc::now();

        tracing::info!("Creating API key: name={}, key_prefix={}", 
            request.name, &request.key_value[..10]);

        let result = sqlx::query(
            r#"
            INSERT INTO api_keys (id, name, key_value, is_active, usage_count, created_at, updated_at)
            VALUES (?, ?, ?, ?, ?, ?, ?)
            "#,
        )
        .bind(key_id.to_string())
        .bind(&request.name)
        .bind(&request.key_value)
        .bind(1)
        .bind(0)
        .bind(to_js_compatible_timestamp(now))
        .bind(to_js_compatible_timestamp(now))
        .execute(&self.pool)
        .await;

        match result {
            Ok(_) => {
                tracing::info!("Successfully created API key with ID: {}", key_id);
            }
            Err(ref e) => {
                tracing::error!("Failed to create API key: {}", e);
            }
        }

        result?;

        Ok(ApiKeyResponse {
            id: key_id,
            name: request.name,
            key_value: request.key_value,
            is_active: true,
            usage_count: 0,
            last_used: None,
            created_at: now,
        })
    }

    pub async fn get_all_api_keys(&self) -> Result<Vec<ApiKeyResponse>> {
        let keys: Vec<ApiKey> = sqlx::query_as(
            r#"
            SELECT id, name, key_value, is_active, usage_count, last_used, created_at, updated_at
            FROM api_keys ORDER BY created_at DESC
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        Ok(keys.into_iter().map(|k| ApiKeyResponse {
            id: k.id,
            name: k.name,
            key_value: k.key_value,
            is_active: k.is_active,
            usage_count: k.usage_count,
            last_used: k.last_used,
            created_at: k.created_at,
        }).collect())
    }

    pub async fn get_api_keys_paginated(&self, page: u32, per_page: u32) -> Result<(Vec<ApiKeyResponse>, u32)> {
        let offset = (page - 1) * per_page;
        
        // 获取总数
        let total: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM api_keys")
            .fetch_one(&self.pool)
            .await?;
        let total_count = total.0 as u32;
        
        // 获取分页数据
        let keys: Vec<ApiKey> = sqlx::query_as(
            r#"
            SELECT id, name, key_value, is_active, usage_count, last_used, created_at, updated_at
            FROM api_keys ORDER BY created_at DESC
            LIMIT ? OFFSET ?
            "#,
        )
        .bind(per_page)
        .bind(offset)
        .fetch_all(&self.pool)
        .await?;

        let api_keys = keys.into_iter().map(|k| ApiKeyResponse {
            id: k.id,
            name: k.name,
            key_value: k.key_value,
            is_active: k.is_active,
            usage_count: k.usage_count,
            last_used: k.last_used,
            created_at: k.created_at,
        }).collect();

        Ok((api_keys, total_count))
    }

    pub async fn update_api_key(&self, key_id: Uuid, request: UpdateApiKeyRequest) -> Result<Option<ApiKeyResponse>> {
        let now = Utc::now();

        let mut query_parts = Vec::new();
        let mut bind_values = Vec::new();

        if let Some(name) = &request.name {
            query_parts.push("name = ?");
            bind_values.push(name.clone());
        }

        if let Some(is_active) = request.is_active {
            query_parts.push("is_active = ?");
            bind_values.push((is_active as i32).to_string());
        }

        if query_parts.is_empty() {
            return self.get_api_key_by_id(key_id).await;
        }

        query_parts.push("updated_at = ?");
        bind_values.push(to_js_compatible_timestamp(now));

        let query = format!(
            "UPDATE api_keys SET {} WHERE id = ?",
            query_parts.join(", ")
        );

        let mut query_builder = sqlx::query(&query);
        for value in bind_values {
            query_builder = query_builder.bind(value);
        }
        query_builder = query_builder.bind(key_id.to_string());

        query_builder.execute(&self.pool).await?;

        self.get_api_key_by_id(key_id).await
    }

    pub async fn delete_api_key(&self, key_id: Uuid) -> Result<bool> {
        let result = sqlx::query(
            "DELETE FROM api_keys WHERE id = ?"
        )
        .bind(key_id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(result.rows_affected() > 0)
    }

    pub async fn get_api_key_by_id(&self, key_id: Uuid) -> Result<Option<ApiKeyResponse>> {
        let key: Option<ApiKey> = sqlx::query_as(
            r#"
            SELECT id, name, key_value, is_active, usage_count, last_used, created_at, updated_at
            FROM api_keys WHERE id = ?
            "#,
        )
        .bind(key_id.to_string())
        .fetch_optional(&self.pool)
        .await?;

        Ok(key.map(|k| ApiKeyResponse {
            id: k.id,
            name: k.name,
            key_value: k.key_value,
            is_active: k.is_active,
            usage_count: k.usage_count,
            last_used: k.last_used,
            created_at: k.created_at,
        }))
    }

    pub async fn increment_usage(&self, key_id: Uuid) -> Result<()> {
        let now = Utc::now();
        sqlx::query(
            "UPDATE api_keys SET usage_count = usage_count + 1, last_used = ? WHERE id = ?"
        )
        .bind(to_js_compatible_timestamp(now))
        .bind(key_id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}