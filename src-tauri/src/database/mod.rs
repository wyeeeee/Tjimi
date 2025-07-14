use sqlx::{SqlitePool, migrate::MigrateDatabase};
use anyhow::Result;

pub mod migrations;

pub async fn init_database() -> Result<SqlitePool> {
    // Use system temp directory to avoid file watching issues
    let temp_dir = std::env::temp_dir();
    let db_path = temp_dir.join("gemini_proxy.db");
    let database_url = format!("sqlite:{}", db_path.display());
    
    if !sqlx::Sqlite::database_exists(&database_url).await? {
        sqlx::Sqlite::create_database(&database_url).await?;
    }
    
    let pool = SqlitePool::connect(&database_url).await?;
    
    // Run migrations
    migrations::run_migrations(&pool).await?;
    
    Ok(pool)
}