use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
    sync::{Arc, Mutex, OnceLock},
    time::Duration,
};

use cached::{Cached, TimedCache};
use log::{debug, warn};
use serde_json::{json, Value};
use tauri_plugin_http::reqwest::{
    header::{HeaderMap, HeaderValue, COOKIE, REFERER, SET_COOKIE, USER_AGENT},
    ClientBuilder, Proxy,
};

use crate::{
    get_cookie_manager,
    network::crypto::{eapi, weapi},
    AppError,
};

static REQUEST_CACHE: OnceLock<Arc<Mutex<TimedCache<String, Response>>>> = OnceLock::new();

fn get_cache() -> &'static Arc<Mutex<TimedCache<String, Response>>> {
    REQUEST_CACHE.get_or_init(|| {
        Arc::new(Mutex::new(TimedCache::with_lifespan_and_capacity(
            300, 1000,
        )))
    })
}

#[derive(Default, Debug)]
pub struct RequestOption {
    pub crypto: Option<String>,
    pub ip: Option<String>,
    pub real_ip: Option<String>,
    pub headers: Option<HeaderMap>,
    pub proxy: Option<String>,
    pub e_r: Option<bool>,
    pub ua: Option<String>,
    pub cache: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct Response {
    #[allow(dead_code)]
    pub status: u16,
    pub body: Value,
    pub cookie: Option<Vec<String>>,
    #[allow(dead_code)]
    pub headers: HeaderMap,
}

fn generate_cache_key(uri: &str, data: &Value) -> String {
    let mut hasher = DefaultHasher::new();
    uri.hash(&mut hasher);
    serde_json::to_string(data)
        .unwrap_or_default()
        .hash(&mut hasher);
    format!("{:x}", hasher.finish())
}

pub async fn create_request(
    uri: &str,
    mut data: Value,
    option: RequestOption,
) -> Result<Response, AppError> {
    let use_cache = option.cache.unwrap_or(true);
    let cache_key = generate_cache_key(uri, &data);

    if use_cache {
        if let Ok(mut cache) = get_cache().lock() {
            if let Some(cached) = cache.cache_get(&cache_key) {
                debug!("Cache hit: {}", cache_key);
                return Ok(cached.clone());
            }
        }
    }

    let mut headers = option.headers.unwrap_or_default();

    if let Some(ip) = option.real_ip.or(option.ip).filter(|s| !s.is_empty()) {
        if let Ok(ip_header_value) = ip.parse::<HeaderValue>() {
            headers.insert("X-Real-IP", ip_header_value.clone());
            headers.insert("X-Forwarded-For", ip_header_value);
        } else {
            warn!("Invalid IP address format '{}', skipping IP headers", ip);
        }
    }

    let cookie_string = get_cookie_manager().get_header_value();
    if !cookie_string.is_empty() {
        let cookie_value = cookie_string
            .parse::<HeaderValue>()
            .map_err(|_| AppError::Cookie("Invalid cookie format".to_string()))?;
        headers.insert(COOKIE, cookie_value);
    }

    let crypto = option.crypto.as_deref().unwrap_or_else(|| "eapi");

    let (mut url, encrypt_data) = match crypto {
        "weapi" => {
            headers.insert(REFERER, "https://music.163.com".parse()?);
            headers.insert(
                USER_AGENT,
                option
                    .ua
                    .unwrap_or_else(|| choose_user_agent(crypto, "pc"))
                    .parse()?,
            );
            (
                format!("{}/weapi/{}", "https://music.163.com", &uri[5..]),
                weapi(&data)?,
            )
        }
        "linuxapi" => {
            headers.insert(
                USER_AGENT,
                option
                    .ua
                    .unwrap_or_else(|| choose_user_agent(crypto, "linux"))
                    .parse()?,
            );
            let unencrypted_data = json!({"method": "POST", "url": format!("{}{}", "https://music.163.com", uri), "params": data});
            (
                format!("{}/api/linux/forward", "https://music.163.com"),
                eapi(uri, &unencrypted_data)?,
            )
        }
        "eapi" | "api" => {
            headers.insert(
                USER_AGENT,
                option
                    .ua
                    .unwrap_or_else(|| choose_user_agent("api", "iphone"))
                    .parse()?,
            );
            if crypto == "eapi" {
                data["e_r"] = option
                    .e_r
                    .or_else(|| data.get("e_r").and_then(|v| v.as_bool()).or(Some(false)))
                    .unwrap_or_default()
                    .into();
                (
                    format!("{}/eapi/{}", "https://interface.music.163.com", &uri[5..]),
                    eapi(uri, &data)?,
                )
            } else {
                (
                    format!("{}{}", "https://interface.music.163.com", uri),
                    data,
                )
            }
        }
        _ => return Err(AppError::Crypto(format!("Unsupported crypto: {}", crypto))),
    };

    let mut client_builder = ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .pool_max_idle_per_host(30)
        .pool_idle_timeout(Duration::from_secs(90))
        .gzip(true);

    if let Some(proxy_url) = &option.proxy {
        client_builder = client_builder.proxy(Proxy::all(proxy_url)?);
    }

    let client = client_builder.build()?;
    url = format!("{}?{}", url, json_to_urlencoded(&encrypt_data));

    let response = client.post(&url).headers(headers).send().await?;

    let status = response.status().as_u16();
    let headers = response.headers().clone();

    let cookies: Vec<String> = headers
        .get_all(SET_COOKIE)
        .iter()
        .filter_map(|v| v.to_str().ok())
        .map(|s| {
            s.split(';')
                .filter(|p| !p.trim().starts_with("Domain="))
                .collect::<Vec<_>>()
                .join("; ")
        })
        .collect();

    get_cookie_manager().update_from_headers(response.headers());

    let body = response.json().await?;

    let answer = Response {
        status,
        body,
        cookie: if cookies.is_empty() {
            None
        } else {
            Some(cookies)
        },
        headers,
    };

    warn!("[{}] {} - {}", crypto, answer.clone().status, uri);

    if status == 200 && use_cache && answer.body.get("code").and_then(|v| v.as_i64()) == Some(200) {
        if let Ok(mut cache) = get_cache().lock() {
            cache.cache_set(cache_key, answer.clone());
        } else {
            warn!("Failed to acquire cache lock for writing");
        }
    }

    Ok(answer)
}

fn choose_user_agent(crypto: &str, ua_type: &str) -> String {
    let user_agent_map = [
        ("weapi", "pc", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0"),
        ("linuxapi", "linux", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.90 Safari/537.36"),
        ("api", "pc", "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Safari/537.36 Chrome/91.0.4472.164 NeteaseMusicDesktop/3.0.18.203152"),
        ("api", "android", "NeteaseMusic/9.1.65.240927161425(9001065);Dalvik/2.1.0 (Linux; U; Android 14; 23013RK75C Build/UKQ1.230804.001)"),
        ("api", "iphone", "NeteaseMusic 9.0.90/5038 (iPhone; iOS 16.2; zh_CN)"),
    ];
    match user_agent_map
        .iter()
        .find(|(key, value, _)| key == &crypto && value == &ua_type)
    {
        Some(x) => x,
        _none => &{
            warn!("Unknown user agent {}/{}", crypto, ua_type);
            ("", "", "")
        },
    }
    .2
    .to_string()
}

fn json_to_urlencoded(data: &Value) -> String {
    let mut serializer = form_urlencoded::Serializer::new(String::new());

    if let Some(map) = data.as_object() {
        for (key, value) in map {
            match value {
                Value::String(s) => {
                    serializer.append_pair(key, s);
                }
                Value::Number(num) => {
                    serializer.append_pair(key, &num.to_string());
                }
                Value::Bool(b) => {
                    serializer.append_pair(key, &b.to_string());
                }
                _ => {}
            }
        }
    }

    serializer.finish()
}
