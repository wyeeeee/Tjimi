use sqlx::{SqlitePool, migrate::MigrateDatabase};
use anyhow::Result;
use std::path::PathBuf;

pub mod migrations;

fn get_database_path() -> Result<PathBuf> {
    #[cfg(target_os = "windows")]
    {
        // Windows: Use AppData/Local directory
        if let Some(local_app_data) = std::env::var_os("LOCALAPPDATA") {
            let app_dir = PathBuf::from(local_app_data).join("Tjimi");
            std::fs::create_dir_all(&app_dir)?;
            Ok(app_dir.join("gemini_proxy.db"))
        } else {
            // Fallback to temp directory
            Ok(std::env::temp_dir().join("gemini_proxy.db"))
        }
    }
    
    #[cfg(target_os = "android")]
    {
        // Android: Use app's internal storage directory
        if let Ok(data_dir) = std::env::var("ANDROID_DATA") {
            let app_dir = PathBuf::from(data_dir).join("data").join("com.wye.tjimi").join("databases");
            std::fs::create_dir_all(&app_dir)?;
            Ok(app_dir.join("gemini_proxy.db"))
        } else {
            // Fallback: try to use files directory
            let app_dir = PathBuf::from("/data/data/com.wye.tjimi/databases");
            std::fs::create_dir_all(&app_dir).unwrap_or_default();
            Ok(app_dir.join("gemini_proxy.db"))
        }
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "android")))]
    {
        // Other platforms: Use temp directory as fallback
        Ok(std::env::temp_dir().join("gemini_proxy.db"))
    }
}

pub async fn init_database() -> Result<SqlitePool> {
    let db_path = get_database_path()?;
    let database_url = format!("sqlite:{}", db_path.display());
    
    if !sqlx::Sqlite::database_exists(&database_url).await? {
        sqlx::Sqlite::create_database(&database_url).await?;
    }
    
    let pool = SqlitePool::connect(&database_url).await?;
    
    // Run migrations
    migrations::run_migrations(&pool).await?;
    
    Ok(pool)
}