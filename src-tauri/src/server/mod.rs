pub mod handlers;
pub mod middleware;

use axum::{
    routing::{get, post},
    Router,
    middleware::from_fn_with_state,
};
use sqlx::SqlitePool;
use std::sync::Arc;
use tower_http::cors::CorsLayer;

pub async fn create_app(pool: SqlitePool) -> Router {
    let app_state = Arc::new(pool);

    let protected_routes = Router::new()
        .route("/v1/models", get(handlers::gemini::list_models))
        .route("/v1/models/*path", post(handlers::gemini::generate_content_v1))
        .route("/v1/models/*path", get(handlers::gemini::get_model_by_path_v1))
        .route("/v1beta/models", get(handlers::gemini::list_models))
        .route("/v1beta/models/*path", post(handlers::gemini::generate_content))
        .route("/v1beta/models/*path", get(handlers::gemini::get_model_by_path))
        .layer(from_fn_with_state(app_state.clone(), middleware::custom_auth_middleware));

    Router::new()
        .route("/health", get(handlers::health::health_check))
        .route("/v1", get(handlers::gemini::api_info))
        .merge(protected_routes)
        .layer(CorsLayer::permissive())
        .with_state(app_state)
}