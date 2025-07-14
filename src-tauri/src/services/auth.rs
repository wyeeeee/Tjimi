use crate::models::{AppSettings, LoginRequest, LoginResponse, ChangePasswordRequest};
use sqlx::SqlitePool;
use anyhow::Result;
use bcrypt::{hash, verify, DEFAULT_COST};
use chrono::{Utc, SecondsFormat};
use uuid::Uuid;

fn to_js_compatible_timestamp(dt: chrono::DateTime<Utc>) -> String {
    // 生成 JavaScript 兼容的时间戳（毫秒精度）
    dt.to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub struct AuthService {
    pool: SqlitePool,
}

impl AuthService {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }

    pub async fn login(&self, request: LoginRequest) -> Result<Option<LoginResponse>> {
        let settings: Option<AppSettings> = sqlx::query_as(
            "SELECT id, password_hash, created_at, updated_at FROM app_settings WHERE id = 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(settings) = settings {
            if verify(&request.password, &settings.password_hash)? {
                let session_token = Uuid::new_v4().to_string();
                return Ok(Some(LoginResponse {
                    success: true,
                    session_token,
                }));
            }
        }

        Ok(None)
    }

    pub async fn change_password(&self, request: ChangePasswordRequest) -> Result<bool> {
        let settings: Option<AppSettings> = sqlx::query_as(
            "SELECT id, password_hash, created_at, updated_at FROM app_settings WHERE id = 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(settings) = settings {
            if verify(&request.current_password, &settings.password_hash)? {
                let new_password_hash = hash(request.new_password, DEFAULT_COST)?;
                let now = Utc::now();

                sqlx::query(
                    "UPDATE app_settings SET password_hash = ?, updated_at = ? WHERE id = 1"
                )
                .bind(new_password_hash)
                .bind(to_js_compatible_timestamp(now))
                .execute(&self.pool)
                .await?;

                return Ok(true);
            }
        }

        Ok(false)
    }

    pub async fn get_default_password_info(&self) -> Result<bool> {
        let settings: Option<AppSettings> = sqlx::query_as(
            "SELECT id, password_hash, created_at, updated_at FROM app_settings WHERE id = 1"
        )
        .fetch_optional(&self.pool)
        .await?;

        if let Some(settings) = settings {
            // Check if it's still using the default password
            return Ok(verify("admin123", &settings.password_hash).unwrap_or(false));
        }

        Ok(false)
    }
}