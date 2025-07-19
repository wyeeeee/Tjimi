use tauri::{AppHandle, Manager};

#[cfg(not(target_os = "android"))]
#[tauri::command]
pub async fn minimize_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.minimize().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(target_os = "android")]
#[tauri::command]
pub async fn minimize_window(_app: AppHandle) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "android"))]
#[tauri::command]
pub async fn maximize_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.maximize().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(target_os = "android")]
#[tauri::command]
pub async fn maximize_window(_app: AppHandle) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "android"))]
#[tauri::command]
pub async fn unmaximize_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.unmaximize().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(target_os = "android")]
#[tauri::command]
pub async fn unmaximize_window(_app: AppHandle) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "android"))]
#[tauri::command]
pub async fn close_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.destroy().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(target_os = "android")]
#[tauri::command]
pub async fn close_window(_app: AppHandle) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "android"))]
#[tauri::command]
pub async fn toggle_maximize_window(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        let is_maximized = window.is_maximized().map_err(|e| e.to_string())?;
        if is_maximized {
            window.unmaximize().map_err(|e| e.to_string())?;
        } else {
            window.maximize().map_err(|e| e.to_string())?;
        }
    }
    Ok(())
}

#[cfg(target_os = "android")]
#[tauri::command]
pub async fn toggle_maximize_window(_app: AppHandle) -> Result<(), String> {
    Ok(())
}

#[cfg(not(target_os = "android"))]
#[tauri::command]
pub async fn is_window_maximized(app: AppHandle) -> Result<bool, String> {
    if let Some(window) = app.get_webview_window("main") {
        return window.is_maximized().map_err(|e| e.to_string());
    }
    Ok(false)
}

#[cfg(target_os = "android")]
#[tauri::command]
pub async fn is_window_maximized(_app: AppHandle) -> Result<bool, String> {
    Ok(false)
}

#[cfg(not(target_os = "android"))]
#[tauri::command]
pub async fn start_drag(app: AppHandle) -> Result<(), String> {
    if let Some(window) = app.get_webview_window("main") {
        window.start_dragging().map_err(|e| e.to_string())?;
    }
    Ok(())
}

#[cfg(target_os = "android")]
#[tauri::command]
pub async fn start_drag(_app: AppHandle) -> Result<(), String> {
    Ok(())
}

#[cfg(target_os = "android")]
#[tauri::command]
pub async fn is_desktop() -> Result<bool, String> {
    Ok(false)
}

#[cfg(not(target_os = "android"))]
#[tauri::command]
pub async fn is_desktop() -> Result<bool, String> {
    Ok(true)
}