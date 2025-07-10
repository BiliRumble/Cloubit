use log::warn;
use rand::random;
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
            .map_err(|_| AppError::Cookie("Invalid cookie format".to_string()))?;
        headers.insert(COOKIE, cookie_value);
    }

    match crypto {
        "weapi" => {
            headers.insert(REFERER, "https://music.163.com".parse()?);
            let cookie_manager = get_cookie_manager();
            let mut cookie_parts = Vec::new();
            let cookies = cookie_manager.get_header_value();
            if !cookies.is_empty() {
                cookie_parts.push(cookies);
            }
            if let Some(csrf) = cookie_manager.get("__csrf") {
                if !csrf.is_empty() {
                    cookie_parts.push(format!("__csrf={}", csrf));
                }
            }
            if !cookie_parts.is_empty() {
                let cookie_string = cookie_parts.join("; ");
                let cookie_value = cookie_string
                    .parse::<HeaderValue>()
                    .map_err(|_| AppError::Network("Invalid cookie format".to_string()))?;
                headers.insert(COOKIE, cookie_value);
            }
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
            let cookie_manager = get_cookie_manager();
            let timestamp = std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_millis() as u64;

            let mut header_map = std::collections::HashMap::new();
            header_map.insert("osver", "16.2".to_string());
            header_map.insert("deviceId", crate::get_device_id().to_string());
            header_map.insert("os", "iPhone OS".to_string());
            header_map.insert("appver", "9.0.90".to_string());
            header_map.insert("versioncode", "140".to_string());
            header_map.insert("mobilename", "".to_string());
            header_map.insert("buildver", (timestamp / 1000).to_string());
            header_map.insert("resolution", "1920x1080".to_string());
            header_map.insert("__csrf", cookie_manager.get("__csrf").unwrap_or_default());
            header_map.insert("channel", "distribution".to_string());
            header_map.insert(
                "requestId",
                format!("{}_{:04}", timestamp, random::<u16>() % 1000),
            );
            if let Some(music_u) = cookie_manager.get("MUSIC_U") {
                if !music_u.is_empty() {
                    header_map.insert("MUSIC_U", music_u);
                }
            } else if let Some(music_a) = cookie_manager.get("MUSIC_A") {
                if !music_a.is_empty() {
                    header_map.insert("MUSIC_A", music_a);
                }
            }
            let cookie_string = header_map
                .iter()
                .map(|(key, value)| {
                    format!(
                        "{}={}",
                        urlencoding::encode(key),
                        urlencoding::encode(value)
                    )
                })
                .collect::<Vec<_>>()
                .join("; ");

            let cookie_value = cookie_string
                .parse::<HeaderValue>()
                .map_err(|_| AppError::Network("Invalid cookie format".to_string()))?;
            headers.insert(COOKIE, cookie_value);

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
