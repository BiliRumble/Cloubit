use log::warn;
use reqwest::header::{COOKIE, HeaderMap, HeaderValue, REFERER, USER_AGENT};

use crate::{error::AppError, models::http::RequestOption, storage::cookie::get_cookie_manager};

fn choose_user_agent(crypto: &str, ua_type: &str) -> &'static str {
    const USER_AGENTS: &[(&str, &str, &str)] = &[
        (
            "weapi",
            "pc",
            "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0",
        ),
        (
            "linuxapi",
            "linux",
            "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.90 Safari/537.36",
        ),
        (
            "api",
            "pc",
            "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Safari/537.36 Chrome/91.0.4472.164 NeteaseMusicDesktop/3.0.18.203152",
        ),
        (
            "api",
            "android",
            "NeteaseMusic/9.1.65.240927161425(9001065);Dalvik/2.1.0 (Linux; U; Android 14; 23013RK75C Build/UKQ1.230804.001)",
        ),
        (
            "api",
            "iphone",
            "NeteaseMusic 9.0.90/5038 (iPhone; iOS 16.2; zh_CN)",
        ),
    ];

    USER_AGENTS
        .iter()
        .find(|(key, value, _)| key == &crypto && value == &ua_type)
        .map(|(_, _, ua)| *ua)
        .unwrap_or_else(|| {
            warn!("Unknown user agent {}/{}", crypto, ua_type);
            ""
        })
}

pub fn build_request_headers(option: &RequestOption, crypto: &str) -> Result<HeaderMap, AppError> {
    let mut headers = option.headers.clone().unwrap_or_default();

    if let Some(ip) = option
        .real_ip
        .as_ref()
        .or(option.ip.as_ref())
        .filter(|s| !s.is_empty())
    {
        match ip.parse::<HeaderValue>() {
            Ok(ip_header_value) => {
                headers.insert("X-Real-IP", ip_header_value.clone());
                headers.insert("X-Forwarded-For", ip_header_value);
            }
            Err(_) => warn!("Invalid IP address format '{}', skipping IP headers", ip),
        }
    }

    let cookie_string = get_cookie_manager().get_header_value();
    if !cookie_string.is_empty() {
        let cookie_value = cookie_string
            .parse::<HeaderValue>()
            .map_err(|_| AppError::Network("Invalid cookie format".to_string()))?;
        headers.insert(COOKIE, cookie_value);
    }

    match crypto {
        "weapi" => {
            headers.insert(REFERER, "https://music.163.com".parse()?);
            headers.insert(
                USER_AGENT,
                option
                    .ua
                    .as_deref()
                    .unwrap_or_else(|| choose_user_agent(crypto, "pc"))
                    .parse()?,
            );
        }
        "linuxapi" => {
            headers.insert(
                USER_AGENT,
                option
                    .ua
                    .as_deref()
                    .unwrap_or_else(|| choose_user_agent(crypto, "linux"))
                    .parse()?,
            );
        }
        "eapi" | "api" => {
            headers.insert(
                USER_AGENT,
                option
                    .ua
                    .as_deref()
                    .unwrap_or_else(|| choose_user_agent("api", "iphone"))
                    .parse()?,
            );
        }
        _ => return Err(AppError::Crypto(format!("Unsupported crypto: {}", crypto))),
    }

    Ok(headers)
}
