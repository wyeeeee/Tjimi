use crate::models::{LoginRequest, LoginResponse, ChangePasswordRequest};
use crate::services::AuthService;
use serde::Serialize;
use tauri::State;
use sqlx::SqlitePool;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct AuthResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[tauri::command]
pub async fn login(
    request: LoginRequest,
    pool: State<'_, SqlitePool>,
) -> Result<AuthResult<LoginResponse>, String> {
    let auth_service = AuthService::new(pool.inner().clone());
    
    match auth_service.login(request).await {
        Ok(Some(login_response)) => Ok(AuthResult {
            success: true,
            data: Some(login_response),
            error: None,
        }),
        Ok(None) => Ok(AuthResult {
            success: false,
            data: None,
            error: Some("密码错误".to_string()),
        }),
        Err(e) => Ok(AuthResult {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn change_password(
    request: ChangePasswordRequest,
    pool: State<'_, SqlitePool>,
) -> Result<AuthResult<bool>, String> {
    let auth_service = AuthService::new(pool.inner().clone());
    
    match auth_service.change_password(request).await {
        Ok(true) => Ok(AuthResult {
            success: true,
            data: Some(true),
            error: None,
        }),
        Ok(false) => Ok(AuthResult {
            success: false,
            data: Some(false),
            error: Some("当前密码错误".to_string()),
        }),
        Err(e) => Ok(AuthResult {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn check_default_password(
    pool: State<'_, SqlitePool>,
) -> Result<AuthResult<bool>, String> {
    let auth_service = AuthService::new(pool.inner().clone());
    
    match auth_service.get_default_password_info().await {
        Ok(is_default) => Ok(AuthResult {
            success: true,
            data: Some(is_default),
            error: None,
        }),
        Err(e) => Ok(AuthResult {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}