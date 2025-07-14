use sqlx::SqlitePool;
use anyhow::Result;

pub async fn run_migrations(pool: &SqlitePool) -> Result<()> {
    // Create app_settings table (single user mode)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS app_settings (
            id INTEGER PRIMARY KEY CHECK (id = 1),
            password_hash TEXT NOT NULL,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Initialize default password if no settings exist
    let exists: (i64,) = sqlx::query_as("SELECT COUNT(*) FROM app_settings")
        .fetch_one(pool)
        .await?;

    if exists.0 == 0 {
        let default_password_hash = bcrypt::hash("admin123", bcrypt::DEFAULT_COST)
            .map_err(|e| sqlx::Error::Protocol(format!("Failed to hash password: {}", e)))?;
        let now = chrono::Utc::now().to_rfc3339_opts(chrono::SecondsFormat::Millis, true);

        sqlx::query(
            "INSERT INTO app_settings (id, password_hash, created_at, updated_at) VALUES (1, ?, ?, ?)"
        )
        .bind(default_password_hash)
        .bind(&now)
        .bind(&now)
        .execute(pool)
        .await?;
    }

    // Create api_keys table (no user_id foreign key needed)
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS api_keys (
            id TEXT PRIMARY KEY,
            name TEXT NOT NULL,
            key_value TEXT NOT NULL,
            is_active INTEGER NOT NULL DEFAULT 1,
            usage_count INTEGER NOT NULL DEFAULT 0,
            last_used TEXT,
            created_at TEXT NOT NULL,
            updated_at TEXT NOT NULL
        )
        "#,
    )
    .execute(pool)
    .await?;

    // Create request_logs table
    sqlx::query(
        r#"
        CREATE TABLE IF NOT EXISTS request_logs (
            id TEXT PRIMARY KEY,
            api_key_id TEXT NOT NULL,
            method TEXT NOT NULL,
            path TEXT NOT NULL,
            status_code INTEGER NOT NULL,
            response_time_ms INTEGER NOT NULL,
            created_at TEXT NOT NULL,
            FOREIGN KEY (api_key_id) REFERENCES api_keys(id)
        )
        "#,
    )
    .execute(pool)
    .await?;

    Ok(())
}