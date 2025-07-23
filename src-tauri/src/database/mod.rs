use sqlx::{SqlitePool, migrate::MigrateDatabase};
use anyhow::Result;
use std::path::PathBuf;
use tauri::Manager;

pub mod migrations;

fn get_database_path(_app_handle: Option<&tauri::AppHandle>) -> Result<PathBuf> {
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
        // Android: Use app data directory for persistent storage
        // This is typically /data/data/com.wye.tjimi/files/
        if let Some(handle) = _app_handle {
            let app_data_dir = handle.path().app_data_dir()
                .map_err(|e| anyhow::anyhow!("Failed to get app data directory: {}", e))?;
            let db_path = app_data_dir.join("gemini_proxy.db");
            
            // Ensure the directory exists
            if let Some(parent) = db_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            
            Ok(db_path)
        } else {
            // Fallback to temp directory if no app handle provided
            let temp_dir = std::env::temp_dir();
            let db_path = temp_dir.join("gemini_proxy.db");
            
            if let Some(parent) = db_path.parent() {
                std::fs::create_dir_all(parent).unwrap_or_default();
            }
            
            Ok(db_path)
        }
    }
    
    #[cfg(not(any(target_os = "windows", target_os = "android")))]
    {
        // Other platforms: Use temp directory as fallback
        Ok(std::env::temp_dir().join("gemini_proxy.db"))
    }
}

pub async fn init_database() -> Result<SqlitePool> {
    let db_path = get_database_path(None)?;
    let database_url = format!("sqlite:{}", db_path.display());
    
    if !sqlx::Sqlite::database_exists(&database_url).await? {
        sqlx::Sqlite::create_database(&database_url).await?;
    }
    
    let pool = SqlitePool::connect(&database_url).await?;
    
    // Run migrations
    migrations::run_migrations(&pool).await?;
    
    Ok(pool)
}

pub async fn init_database_with_app_handle(app_handle: &tauri::AppHandle) -> Result<SqlitePool> {
    let db_path = get_database_path(Some(app_handle))?;
    let database_url = format!("sqlite:{}", db_path.display());
    
    tracing::info!("Initializing database at: {}", db_path.display());
    
    if !sqlx::Sqlite::database_exists(&database_url).await? {
        sqlx::Sqlite::create_database(&database_url).await?;
    }
    
    let pool = SqlitePool::connect(&database_url).await?;
    
    // Run migrations
    migrations::run_migrations(&pool).await?;
    
    Ok(pool)
}