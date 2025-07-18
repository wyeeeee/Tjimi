use tauri::State;
use crate::services::CustomAuthService;

#[tauri::command]
pub async fn set_custom_auth_key(
    service: State<'_, CustomAuthService>,
    key: String,
) -> Result<(), String> {
    service
        .set_custom_key(&key)
        .await
        .map_err(|e| format!("Failed to set custom auth key: {}", e))
}

#[tauri::command]
pub async fn clear_custom_auth_key(
    service: State<'_, CustomAuthService>,
) -> Result<(), String> {
    service
        .clear_custom_key()
        .await
        .map_err(|e| format!("Failed to clear custom auth key: {}", e))
}

#[tauri::command]
pub async fn has_custom_auth_key(
    service: State<'_, CustomAuthService>,
) -> Result<bool, String> {
    service
        .has_custom_key()
        .await
        .map_err(|e| format!("Failed to check custom auth key: {}", e))
}

#[tauri::command]
pub async fn validate_custom_auth_key(
    service: State<'_, CustomAuthService>,
    key: String,
) -> Result<bool, String> {
    service
        .validate_custom_key(&key)
        .await
        .map_err(|e| format!("Failed to validate custom auth key: {}", e))
}