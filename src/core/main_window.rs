use crate::{audio::engine::get_backend, error::AppError};

slint::include_modules!();

// 意义不明了属于是
pub fn run() -> Result<Application, AppError> {
    let main_window = Application::new()?;

    main_window.on_play_audio({
        let backend = get_backend();
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

    main_window.on_test_hide({
        let window_weak = main_window.as_weak();
        move || {
            if let Some(window) = window_weak.upgrade() {
                let _ = window.window().hide().ok();
                std::thread::sleep(std::time::Duration::from_secs(3));
                let _ = window.window().show().ok();
            }
        }
    });

    main_window.show()?;
    Ok(main_window)
}
