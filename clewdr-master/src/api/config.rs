use axum::Json;
use axum_auth::AuthBearer;
use serde_json::json;
use wreq::StatusCode;

use crate::config::{CLEWDR_CONFIG, ClewdrConfig};

/// API endpoint to retrieve the application configuration
/// Returns the config as JSON with sensitive fields removed
///
/// # Arguments
/// * `t` - Auth bearer token for admin authentication
///
/// # Returns
/// * `Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)>` - Config on success, error response on failure
pub async fn api_get_config(
    AuthBearer(t): AuthBearer,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if !CLEWDR_CONFIG.load().admin_auth(&t) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Unauthorized"
            })),
        ));
    }

    let mut config_json = json!(CLEWDR_CONFIG.load().as_ref());
    // remove cookie_array and wasted_cookie
    if let Some(obj) = config_json.as_object_mut() {
        obj.remove("cookie_array");
        obj.remove("wasted_cookie");
        obj.remove("gemini_keys");
        obj["vertex"]["credential"] = "placeholder".into();
    }

    Ok(Json(config_json))
}

/// API endpoint to update the application configuration
/// Validates and stores the provided configuration
///
/// # Arguments
/// * `t` - Auth bearer token for admin authentication
/// * `c` - New configuration data as JSON
///
/// # Returns
/// * `Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)>` - Success message on success, error response on failure
pub async fn api_post_config(
    AuthBearer(t): AuthBearer,
    Json(c): Json<ClewdrConfig>,
) -> Result<Json<serde_json::Value>, (StatusCode, Json<serde_json::Value>)> {
    if !CLEWDR_CONFIG.load().admin_auth(&t) {
        return Err((
            StatusCode::UNAUTHORIZED,
            Json(serde_json::json!({
                "error": "Unauthorized"
            })),
        ));
    }
    let c = c.validate();
    // update config
    CLEWDR_CONFIG.rcu(|old_c| {
        let mut new_c = ClewdrConfig::clone(&c);
        // add cookie_array and wasted_cookie
        new_c.cookie_array = old_c.cookie_array.to_owned();
        new_c.wasted_cookie = old_c.wasted_cookie.to_owned();
        new_c.gemini_keys = old_c.gemini_keys.to_owned();
        if new_c.vertex.credential.is_none() {
            new_c.vertex.credential = old_c.vertex.credential.to_owned();
        }
        new_c
    });
    if let Err(e) = CLEWDR_CONFIG.load().save().await {
        return Err((
            StatusCode::INTERNAL_SERVER_ERROR,
            Json(serde_json::json!({
                "error": format!("Failed to save config: {}", e)
            })),
        ));
    }

    Ok(Json(serde_json::json!({
        "message": "Config updated successfully",
        "config": c
    })))
}
