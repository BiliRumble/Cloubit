slint::include_modules!();

mod audio;
mod crash_handler;
mod error;
mod models;
mod network;
mod service;
mod storage;

use audio::engine::get_backend;
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

    // TODO: 调整设计
    let main_window = MainWindow::new().expect("Could not create window");
    let backend = get_backend();

    main_window.on_play_audio({
        move |url| {
            if !url.trim().is_empty() {
                if let Err(e) = backend
                    .command_sender
                    .send(crate::models::audio::BackendState::Set(url.to_string()))
                {
                    log::error!("Failed to send audio command: {}", e);
                }
            }
        }
    });

    main_window.on_pause_audio({
        move || {
            if let Err(e) = backend
                .command_sender
                .send(crate::models::audio::BackendState::Play(false))
            {
                log::error!("Failed to send audio command: {}", e);
            }
        }
    });

    main_window.on_resume_audio({
        move || {
            if let Err(e) = backend
                .command_sender
                .send(crate::models::audio::BackendState::Play(true))
            {
                log::error!("Failed to send audio command: {}", e);
            }
        }
    });

    // 需要溢出检查
    main_window.on_seek_audio({
        move |pos| {
            if !pos.trim().is_empty() {
                if let Ok(pos) = pos.parse::<u64>() {
                    if let Err(e) = backend
                        .command_sender
                        .send(crate::models::audio::BackendState::Seek(pos))
                    {
                        log::error!("Failed to send audio command: {}", e);
                    }
                }
            }
        }
    });

    main_window.run().expect("Failed to run application");
}
