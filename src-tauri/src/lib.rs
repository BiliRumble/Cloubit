use std::env;

use tauri::http::HeaderMap;
use tauri_plugin_http::reqwest::header::SET_COOKIE;

mod network;
mod service;
mod storage;

mod error;
use error::AppError;

use network::device::get_device_id;
use service::auth::register_anonimous;
use storage::cookie::get_cookie_manager;
use storage::database::get_db;

async fn init_config() -> Result<(), AppError> {
    let response = register_anonimous().await?;

    if let Some(cookies) = response.cookie {
        let mut headers = HeaderMap::new();
        for cookie in cookies {
            if let Ok(header_value) = cookie.parse() {
                headers.append(SET_COOKIE, header_value);
            }
        }
        get_cookie_manager().update_from_headers(&headers);
    }

    Ok(())
}

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    env_logger::init();
    env::set_var("RUST_LOG", "debug");

    tauri::Builder::default()
        .setup(|app| {
            let _ = app.handle();
            tauri::async_runtime::spawn(async move {
                if get_cookie_manager().get_header_value().is_empty() {
                    if let Err(e) = init_config().await {
                        log::error!("Failed to initialize config: {}", e);
                    }
                }
            });
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
