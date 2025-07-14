use serde::{Deserialize, Serialize};
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RequestLog {
    pub id: Uuid,
    pub api_key_id: Uuid,
    pub method: String,
    pub path: String,
    pub status_code: i32,
    pub response_time_ms: i64,
    pub created_at: DateTime<Utc>,
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for RequestLog {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;
        
        let id_str: String = row.try_get("id")?;
        let id = Uuid::parse_str(&id_str)
            .map_err(|e| sqlx::Error::ColumnDecode {
                index: "id".to_string(),
                source: Box::new(e),
            })?;
        
        let api_key_id_str: String = row.try_get("api_key_id")?;
        let api_key_id = Uuid::parse_str(&api_key_id_str)
            .map_err(|e| sqlx::Error::ColumnDecode {
                index: "api_key_id".to_string(),
                source: Box::new(e),
            })?;
        
        let created_at_str: String = row.try_get("created_at")?;
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| sqlx::Error::ColumnDecode {
                index: "created_at".to_string(),
                source: Box::new(e),
            })?;
        
        Ok(RequestLog {
            id,
            api_key_id,
            method: row.try_get("method")?,
            path: row.try_get("path")?,
            status_code: row.try_get("status_code")?,
            response_time_ms: row.try_get("response_time_ms")?,
            created_at,
        })
    }
}

#[derive(Debug, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct RequestLogResponse {
    pub id: Uuid,
    pub api_key_name: String,
    pub method: String,
    pub path: String,
    pub status_code: i32,
    pub response_time_ms: i64,
    pub created_at: DateTime<Utc>,
}

impl sqlx::FromRow<'_, sqlx::sqlite::SqliteRow> for RequestLogResponse {
    fn from_row(row: &sqlx::sqlite::SqliteRow) -> Result<Self, sqlx::Error> {
        use sqlx::Row;
        
        let id_str: String = row.try_get("id")?;
        let id = Uuid::parse_str(&id_str)
            .map_err(|e| sqlx::Error::ColumnDecode {
                index: "id".to_string(),
                source: Box::new(e),
            })?;
        
        let created_at_str: String = row.try_get("created_at")?;
        let created_at = DateTime::parse_from_rfc3339(&created_at_str)
            .map(|dt| dt.with_timezone(&Utc))
            .map_err(|e| sqlx::Error::ColumnDecode {
                index: "created_at".to_string(),
                source: Box::new(e),
            })?;
        
        Ok(RequestLogResponse {
            id,
            api_key_name: row.try_get("api_key_name")?,
            method: row.try_get("method")?,
            path: row.try_get("path")?,
            status_code: row.try_get("status_code")?,
            response_time_ms: row.try_get("response_time_ms")?,
            created_at,
        })
    }
}