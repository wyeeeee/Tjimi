pub mod handlers;

use axum::{
    routing::{get, post},
    Router,
};
use sqlx::SqlitePool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub async fn create_app(pool: SqlitePool) -> Router {
    let app_state = Arc::new(pool);

    Router::new()
        .route("/v1/models", get(handlers::gemini::list_models))
        .route("/v1/models/:model/generateContent", post(handlers::gemini::generate_content))
        .route("/v1/models/:model/streamGenerateContent", post(handlers::gemini::stream_generate_content))
        .route("/v1/models/:model", get(handlers::gemini::get_model))
        .route("/health", get(handlers::health::health_check))
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}