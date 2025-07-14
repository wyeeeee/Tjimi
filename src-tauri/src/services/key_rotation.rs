use crate::models::ApiKey;
use sqlx::SqlitePool;
use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct KeyRotationService {
    pool: SqlitePool,
    current_key_index: Arc<RwLock<usize>>,
}

impl KeyRotationService {
    pub fn new(pool: SqlitePool) -> Self {
        Self {
            pool,
            current_key_index: Arc::new(RwLock::new(0)),
        }
    }

    pub async fn get_next_active_key(&self) -> Result<Option<ApiKey>> {
        let keys: Vec<ApiKey> = sqlx::query_as(
            r#"
            SELECT id, name, key_value, is_active, usage_count, last_used, created_at, updated_at
            FROM api_keys WHERE is_active = 1 ORDER BY usage_count ASC, last_used ASC NULLS FIRST
            "#,
        )
        .fetch_all(&self.pool)
        .await?;

        if keys.is_empty() {
            return Ok(None);
        }

        let mut current_index = self.current_key_index.write().await;
        let key = keys.get(*current_index).cloned();
        
        *current_index = (*current_index + 1) % keys.len();
        
        Ok(key)
    }

    pub async fn get_active_keys_count(&self) -> Result<i64> {
        let count: (i64,) = sqlx::query_as(
            "SELECT COUNT(*) FROM api_keys WHERE is_active = 1"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(count.0)
    }

    pub async fn mark_key_as_failed(&self, key_id: uuid::Uuid) -> Result<()> {
        sqlx::query(
            "UPDATE api_keys SET is_active = 0 WHERE id = ?"
        )
        .bind(key_id.to_string())
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}