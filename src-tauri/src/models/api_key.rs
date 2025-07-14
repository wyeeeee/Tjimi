use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ApiKey {
    pub id: Uuid,
    pub name: String,
    pub key_value: String,
    pub is_active: bool,
    pub usage_count: i64,
    pub last_used: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for ApiKey {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;
        
        let id_str: String = row.try_get("id")?;
        let id = Uuid::parse_str(&id_str)
            .map_err(|e| sqlx::Error::ColumnDecode {
                index: "id".to_string(),
                source: Box::new(e),
            })?;
        
        let is_active_int: i32 = row.try_get("is_active")?;
        let is_active = is_active_int != 0;
        
        let last_used_str: Option<String> = row.try_get("last_used")?;
        let last_used = last_used_str
            .map(|s| DateTime::parse_from_rfc3339(&s)
                .map(|dt| dt.with_timezone(&Utc))
                .map_err(|e| sqlx::Error::ColumnDecode {
                    index: "last_used".to_string(),
                    source: Box::new(e),
                }))
            .transpose()?;
        
        let created_at_str: String = row.try_get("created_at")?;
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| sqlx::Error::ColumnDecode {
                index: "created_at".to_string(),
                source: Box::new(e),
            })?;
        
        let updated_at_str: String = row.try_get("updated_at")?;
        let updated_at = DateTime::parse_from_rfc3339(&updated_at_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| sqlx::Error::ColumnDecode {
                index: "updated_at".to_string(),
                source: Box::new(e),
            })?;
        
        Ok(ApiKey {
            id,
            name: row.try_get("name")?,
            key_value: row.try_get("key_value")?,
            is_active,
            usage_count: row.try_get("usage_count")?,
            last_used,
            created_at,
            updated_at,
        })
    }
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct CreateApiKeyRequest {
    pub name: String,
    pub key_value: String,
}

#[derive(Debug, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct UpdateApiKeyRequest {
    pub name: Option<String>,
    pub is_active: Option<bool>,
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ApiKeyResponse {
    pub id: Uuid,
    pub name: String,
    pub key_value: String,
    pub is_active: bool,
    pub usage_count: i64,
    pub last_used: Option<DateTime<Utc>>,
    pub created_at: DateTime<Utc>,
}