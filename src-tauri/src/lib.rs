mod models;
mod services;
mod database;
mod commands;
mod server;

use database::init_database_with_app_handle;
use server::create_app;
use commands::*;
use services::CustomAuthService;
use tauri::Manager;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    // Initialize logging
    tracing_subscriber::fmt::init();
    
    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .setup(|app| {
            let app_handle = app.handle().clone();
            
            // Initialize database synchronously in setup
            tauri::async_runtime::block_on(async move {
                match init_database_with_app_handle(&app_handle).await {
                    Ok(pool) => {
                        tracing::info!("Database initialized successfully");
                        // Manage the pool and services in the app state
                        app_handle.manage(pool.clone());
                        app_handle.manage(CustomAuthService::new(pool.clone()));
                        
                        // Start HTTP server in background
                        let server_pool = pool.clone();
                        tauri::async_runtime::spawn(async move {
                            let server_app = create_app(server_pool).await;
                            let listener = tokio::net::TcpListener::bind("0.0.0.0:5675")
                                .await
                                .expect("Failed to bind to port 5675");
                            
                            tracing::info!("Gemini proxy server listening on http://0.0.0.0:5675");
                            
                            axum::serve(listener, server_app)
                                .await
                                .expect("Failed to start server");
                        });
                    }
                    Err(e) => {
                        tracing::error!("Failed to initialize database: {}", e);
                        std::process::exit(1);
                    }
                }
            });
            
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            login,
            change_password,
            check_default_password,
            create_api_key,
            get_all_api_keys,
            get_api_keys_paginated,
            update_api_key,
            delete_api_key,
            get_request_logs,
            get_request_logs_paginated,
            get_usage_stats,
            get_api_key_today_requests,
            set_custom_auth_key,
            reset_custom_auth_key,
            clear_custom_auth_key,
            has_custom_auth_key,
            validate_custom_auth_key,
            minimize_window,
            maximize_window,
            unmaximize_window,
            close_window,
            toggle_maximize_window,
            is_window_maximized,
            start_drag,
            is_desktop,
            get_retry_count,
            set_retry_count
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
