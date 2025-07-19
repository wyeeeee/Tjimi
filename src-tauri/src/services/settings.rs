use sqlx::SqlitePool;
use anyhow::Result;

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
}