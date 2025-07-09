slint::include_modules!();

mod crash_handler;
mod error;
mod models;
mod network;
mod service;
mod storage;

use error::AppError;
use network::device::get_device_id;
use reqwest::header::{HeaderMap, SET_COOKIE};
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

fn main() {
    env_logger::init();

    let crash_handler = crash_handler::CrashHandler::new();
    crash_handler.install();
    crash_handler.cleanup_old_crashes(3);

    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    if get_cookie_manager().get_header_value().is_empty() {
        rt.block_on(async {
            if let Err(e) = init_config().await {
                log::error!("Failed to initialize config: {}", e);
            }
        });
    }

    // TODO
    let _ = MainWindow::new().expect("Could not create window").run();
}
