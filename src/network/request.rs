use std::{sync::OnceLock, time::Duration};

use cached::Cached;
use log::{debug, warn};
use reqwest::{
    ClientBuilder, Proxy,
    header::{HeaderMap, SET_COOKIE},
};
use serde_json::{Value, json};

use crate::{
    AppError, get_cookie_manager,
    models::http::{RequestOption, Response},
    network::{
        crypto::{eapi, weapi},
        header::build_request_headers,
    },
    storage::cache::{generate_cache_key, get_cache},
};

static HTTP_CLIENT: OnceLock<reqwest::Client> = OnceLock::new();

fn get_http_client() -> &'static reqwest::Client {
    HTTP_CLIENT.get_or_init(|| {
        ClientBuilder::new()
            .timeout(Duration::from_secs(15))
            .connect_timeout(Duration::from_secs(10))
            .pool_max_idle_per_host(100)
            .pool_idle_timeout(Duration::from_secs(90))
            .gzip(true)
            .build()
            .expect("Failed to create HTTP client")
    })
}

fn build_request_url_and_data(
    uri: &str,
    mut data: Value,
    crypto: &str,
    option: &RequestOption,
) -> Result<(String, Value), AppError> {
    match crypto {
        "weapi" => Ok((
            format!("https://music.163.com/weapi/{}", &uri[5..]),
            weapi(&data)?,
        )),
        "linuxapi" => {
            let unencrypted_data = json!({
                "method": "POST",
                "url": format!("https://music.163.com{}", uri),
                "params": data
            });
            Ok((
                "https://music.163.com/api/linux/forward".to_string(),
                eapi(uri, &unencrypted_data)?,
            ))
        }
        "eapi" => {
            data["e_r"] = option
                .e_r
                .or_else(|| data.get("e_r").and_then(|v| v.as_bool()).or(Some(false)))
                .unwrap_or_default()
                .into();
            Ok((
                format!("https://interface.music.163.com/eapi/{}", &uri[5..]),
                eapi(uri, &data)?,
            ))
        }
        "api" => Ok((format!("https://interface.music.163.com{}", uri), data)),
        _ => Err(AppError::Crypto(format!("Unsupported crypto: {}", crypto))),
    }
}

fn process_response_cookies(headers: &HeaderMap) -> Option<Vec<String>> {
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

    if cookies.is_empty() {
        None
    } else {
        Some(cookies)
    }
}

pub async fn create_request(
    uri: &str,
    data: Value,
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

    let crypto = option.crypto.as_deref().unwrap_or("eapi");
    let headers = build_request_headers(&option, crypto)?;
    let (url, encrypt_data) = build_request_url_and_data(uri, data, crypto, &option)?;

    let client = if let Some(proxy_url) = &option.proxy {
        ClientBuilder::new()
            .timeout(Duration::from_secs(15))
            .connect_timeout(Duration::from_secs(10))
            .proxy(Proxy::all(proxy_url)?)
            .gzip(true)
            .build()?
    } else {
        get_http_client().clone()
    };

    let final_url = format!("{}?{}", url, json_to_urlencoded(&encrypt_data));

    let response = client.post(&final_url).headers(headers).send().await?;

    let status = response.status().as_u16();
    let response_headers = response.headers().clone();
    let cookies = process_response_cookies(&response_headers);

    get_cookie_manager().update_from_headers(&response_headers);

    let body = response.json().await?;

    let answer = Response {
        status,
        body,
        cookie: cookies,
        headers: response_headers,
    };

    debug!("[{}] {} - {}", crypto, status, uri);

    if status == 200 && use_cache && answer.body.get("code").and_then(|v| v.as_i64()) == Some(200) {
        if let Ok(mut cache) = get_cache().lock() {
            cache.cache_set(cache_key, answer.clone());
        } else {
            warn!("Failed to acquire cache lock for writing");
        }
    }

    Ok(answer)
}

fn json_to_urlencoded(data: &Value) -> String {
    let mut serializer = form_urlencoded::Serializer::new(String::new());

    if let Some(map) = data.as_object() {
        for (key, value) in map {
            let value_str = match value {
                Value::String(s) => s.clone(),
                Value::Number(num) => num.to_string(),
                Value::Bool(b) => b.to_string(),
                _ => continue,
            };
            serializer.append_pair(key, &value_str);
        }
    }

    serializer.finish()
}
