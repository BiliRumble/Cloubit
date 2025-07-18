use crate::{
    core::{main_window, tray},
    error::AppError,
    service::auth::register_anonimous,
    storage::cookie::get_cookie_manager,
};

pub fn run() -> Result<(), AppError> {
    let rt = tokio::runtime::Runtime::new().expect("Failed to create Tokio runtime");

    if get_cookie_manager().get_header_value().is_empty() {
        rt.block_on(async {
            if let Err(e) = init_config().await {
                log::error!("Failed to initialize config: {}", e);
            }
        });
    }

    main_window::run()?;
    tray::init()?;
    Ok(())
}

async fn init_config() -> Result<(), AppError> {
    let response = register_anonimous().await?;

    if let Some(cookies) = response.cookie {
        let mut headers = reqwest::header::HeaderMap::new();
        for cookie in cookies {
            if let Ok(header_value) = cookie.parse() {
                headers.append(reqwest::header::SET_COOKIE, header_value);
            }
        }
        get_cookie_manager().update_from_headers(&headers);
    }

    Ok(())
}
