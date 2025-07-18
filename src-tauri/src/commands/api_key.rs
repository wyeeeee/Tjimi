use crate::models::{CreateApiKeyRequest, UpdateApiKeyRequest, ApiKeyResponse};
use crate::services::ApiKeyService;
use serde::Serialize;
use tauri::State;
use uuid::Uuid;
use sqlx::SqlitePool;

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyResult<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct PaginatedApiKeysResponse {
    pub api_keys: Vec<ApiKeyResponse>,
    pub total_count: u32,
    pub page: u32,
    pub per_page: u32,
    pub total_pages: u32,
}

#[tauri::command]
pub async fn create_api_key(
    request: CreateApiKeyRequest,
    pool: State<'_, SqlitePool>,
) -> Result<ApiKeyResult<ApiKeyResponse>, String> {
    let api_key_service = ApiKeyService::new(pool.inner().clone());
    
    match api_key_service.create_api_key(request).await {
        Ok(api_key) => Ok(ApiKeyResult {
            success: true,
            data: Some(api_key),
            error: None,
        }),
        Err(e) => Ok(ApiKeyResult {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn get_all_api_keys(
    pool: State<'_, SqlitePool>,
) -> Result<ApiKeyResult<Vec<ApiKeyResponse>>, String> {
    let api_key_service = ApiKeyService::new(pool.inner().clone());
    
    match api_key_service.get_all_api_keys().await {
        Ok(api_keys) => Ok(ApiKeyResult {
            success: true,
            data: Some(api_keys),
            error: None,
        }),
        Err(e) => Ok(ApiKeyResult {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn get_api_keys_paginated(
    page: Option<u32>,
    per_page: Option<u32>,
    pool: State<'_, SqlitePool>,
) -> Result<ApiKeyResult<PaginatedApiKeysResponse>, String> {
    let page = page.unwrap_or(1);
    let per_page = per_page.unwrap_or(20).min(100); // 最多100条
    
    let api_key_service = ApiKeyService::new(pool.inner().clone());
    
    match api_key_service.get_api_keys_paginated(page, per_page).await {
        Ok((api_keys, total_count)) => {
            let total_pages = (total_count + per_page - 1) / per_page;
            
            Ok(ApiKeyResult {
                success: true,
                data: Some(PaginatedApiKeysResponse {
                    api_keys,
                    total_count,
                    page,
                    per_page,
                    total_pages,
                }),
                error: None,
            })
        },
        Err(e) => Ok(ApiKeyResult {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn update_api_key(
    keyId: String,
    request: UpdateApiKeyRequest,
    pool: State<'_, SqlitePool>,
) -> Result<ApiKeyResult<ApiKeyResponse>, String> {
    let api_key_service = ApiKeyService::new(pool.inner().clone());
    let key_uuid = Uuid::parse_str(&keyId).map_err(|e| e.to_string())?;
    
    match api_key_service.update_api_key(key_uuid, request).await {
        Ok(Some(api_key)) => Ok(ApiKeyResult {
            success: true,
            data: Some(api_key),
            error: None,
        }),
        Ok(None) => Ok(ApiKeyResult {
            success: false,
            data: None,
            error: Some("API key not found".to_string()),
        }),
        Err(e) => Ok(ApiKeyResult {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}

#[tauri::command]
pub async fn delete_api_key(
    keyId: String,
    pool: State<'_, SqlitePool>,
) -> Result<ApiKeyResult<bool>, String> {
    let api_key_service = ApiKeyService::new(pool.inner().clone());
    let key_uuid = Uuid::parse_str(&keyId).map_err(|e| e.to_string())?;
    
    match api_key_service.delete_api_key(key_uuid).await {
        Ok(deleted) => Ok(ApiKeyResult {
            success: true,
            data: Some(deleted),
            error: None,
        }),
        Err(e) => Ok(ApiKeyResult {
            success: false,
            data: None,
            error: Some(e.to_string()),
        }),
    }
}