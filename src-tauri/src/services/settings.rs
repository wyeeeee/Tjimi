use sqlx::SqlitePool;
use anyhow::Result;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProxySettings {
    pub enabled: bool,
    pub proxy_type: String,
    pub host: Option<String>,
    pub port: Option<i32>,
    pub username: Option<String>,
    pub password: Option<String>,
}

impl Default for ProxySettings {
    fn default() -> Self {
        Self {
            enabled: false,
            proxy_type: "http".to_string(),
            host: None,
            port: None,
            username: None,
            password: None,
        }
    }
}

pub struct SettingsService {
    pool: SqlitePool,
}

impl SettingsService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn get_retry_count(&self) -> Result<i32> {
        let result: (Option<i32>,) = sqlx::query_as(
            "SELECT retry_count FROM app_settings WHERE id = 1"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(result.0.unwrap_or(3)) // Default to 3 if not set
    }

    pub async fn set_retry_count(&self, retry_count: i32) -> Result<()> {
        // Ensure retry_count is at least 1, no upper limit
        let retry_count = retry_count.max(1);
        
        sqlx::query(
            "UPDATE app_settings SET retry_count = ?, updated_at = ? WHERE id = 1"
        )
        .bind(retry_count)
        .bind(chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
        .execute(&self.pool)
        .await?;

        Ok(())
    }

    pub async fn get_proxy_settings(&self) -> Result<ProxySettings> {
        let result: (Option<i32>, Option<String>, Option<String>, Option<i32>, Option<String>, Option<String>) = sqlx::query_as(
            "SELECT proxy_enabled, proxy_type, proxy_host, proxy_port, proxy_username, proxy_password FROM app_settings WHERE id = 1"
        )
        .fetch_one(&self.pool)
        .await?;

        Ok(ProxySettings {
            enabled: result.0.unwrap_or(0) == 1,
            proxy_type: result.1.unwrap_or_else(|| "http".to_string()),
            host: result.2,
            port: result.3,
            username: result.4,
            password: result.5,
        })
    }

    pub async fn set_proxy_settings(&self, settings: &ProxySettings) -> Result<()> {
        sqlx::query(
            "UPDATE app_settings SET proxy_enabled = ?, proxy_type = ?, proxy_host = ?, proxy_port = ?, proxy_username = ?, proxy_password = ?, updated_at = ? WHERE id = 1"
        )
        .bind(if settings.enabled { 1 } else { 0 })
        .bind(&settings.proxy_type)
        .bind(&settings.host)
        .bind(settings.port)
        .bind(&settings.username)
        .bind(&settings.password)
        .bind(chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true))
        .execute(&self.pool)
        .await?;

        Ok(())
    }
}