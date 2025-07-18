use sqlx::SqlitePool;
use anyhow::Result;
use sha2::{Sha256, Digest};

pub struct CustomAuthService {
    pool: SqlitePool,
}

impl CustomAuthService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn set_custom_key(&self, key: &str) -> Result<()> {
        let key_hash = self.hash_key(key);
        let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        
        sqlx::query(
            "UPDATE app_settings SET custom_auth_key = ?, updated_at = ? WHERE id = 1"
        )
        .bind(key_hash)
        .bind(now)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn get_custom_key(&self) -> Result<Option<String>> {
        let result = sqlx::query_as::<_, (Option<String>,)>(
            "SELECT custom_auth_key FROM app_settings WHERE id = 1"
        )
        .fetch_optional(&self.pool)
        .await?;
        
        Ok(result.and_then(|r| r.0))
    }

    pub async fn validate_custom_key(&self, key: &str) -> Result<bool> {
        let key_hash = self.hash_key(key);
        
        let stored_key = self.get_custom_key().await?;
        
        match stored_key {
            Some(stored_hash) => Ok(stored_hash == key_hash),
            None => Ok(false), // 如果没有设置自定义秘钥，拒绝所有请求
        }
    }

    pub async fn clear_custom_key(&self) -> Result<()> {
        let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);
        
        sqlx::query(
            "UPDATE app_settings SET custom_auth_key = NULL, updated_at = ? WHERE id = 1"
        )
        .bind(now)
        .execute(&self.pool)
        .await?;
        
        Ok(())
    }

    pub async fn has_custom_key(&self) -> Result<bool> {
        let result = self.get_custom_key().await?;
        Ok(result.is_some())
    }

    fn hash_key(&self, key: &str) -> String {
        let mut hasher = Sha256::new();
        hasher.update(key.as_bytes());
        format!("{:x}", hasher.finalize())
    }
}