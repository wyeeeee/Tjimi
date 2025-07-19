use crate::services::SettingsService;
use tauri::State;
use sqlx::SqlitePool;

#[tauri::command]
pub async fn get_retry_count(pool: State<'_, SqlitePool>) -> Result<i32, String> {
    let settings_service = SettingsService::new(pool.inner().clone());
    settings_service.get_retry_count().await
        .map_err(|e| e.to_string())
}

#[tauri::command]
pub async fn set_retry_count(retry_count: i32, pool: State<'_, SqlitePool>) -> Result<(), String> {
    let settings_service = SettingsService::new(pool.inner().clone());
    settings_service.set_retry_count(retry_count).await
        .map_err(|e| e.to_string())
}