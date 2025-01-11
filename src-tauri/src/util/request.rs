use crate::util::crypto::{eapi, weapi};
use crate::util::os::choose_os;
use crate::util::text::{choose_user_agent, cookie_obj_to_string, generate_random_hex_string, json_to_urlencoded, map_to_cookie_header, wnmcid};
use crate::{error, ANONYMOUS_TOKEN, CONFIG, DEVICE_ID};
use chrono::Utc;
use reqwest::header::{HeaderMap, COOKIE, REFERER, SET_COOKIE, USER_AGENT};
use reqwest::{ClientBuilder, Proxy};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::ops::Add;
use std::time::Duration;


pub struct RequestOption {
    pub crypto: Option<String>,
    pub ip: Option<String>,
    pub real_ip: Option<String>,
    pub headers: Option<HeaderMap>,
    pub cookie: Option<Value>,
    pub proxy: Option<String>,
    pub e_r: Option<bool>,
    pub ua: Option<String>,
}


#[derive(Deserialize)]
pub struct QueryOption {
    pub crypto: Option<String>,
    pub real_ip: Option<String>,
    pub cookie: Option<String>,
    pub proxy: Option<String>,
    pub e_r: Option<bool>,
    pub ua: Option<String>,
}


pub fn create_request_option(header: HeaderMap, option: &QueryOption, cypto: &str) -> RequestOption {
    RequestOption {
        crypto: option.crypto.clone().or(Some(cypto.to_string())),
        cookie: option.cookie.clone().and_then(|v| serde_json::from_str(&v).ok()),
        ua: option.ua.clone().or(Some("".to_string())),
        ip: None,
        real_ip: option.real_ip.clone().or(None),
        proxy: option.proxy.clone().or(None),
        headers: None,
        e_r: option.e_r.or(None),
    }
}


#[macro_export]
macro_rules! define_request_struct {
    ($name:ident, { $($field_name:ident: $field_type:ty),* $(,)? }) => {
        #[derive(Deserialize)]
        pub struct $name {
            // 特定字段
            $(pub $field_name: $field_type,)*

            // 通用字段
            #[serde(flatten)]
            pub common: QueryOption,

        }
    };
}

#[macro_export]
macro_rules! extract_headers {
    ($req:expr) => {{
        let mut headermap = HeaderMap::new();
        for (key, value) in $req.headers().iter() {
            if let Ok(header_name) = HeaderName::from_bytes(key.as_str().as_bytes()) {
                if let Ok(header_value) = HeaderValue::from_str(value.to_str().unwrap_or("")) {
                    headermap.insert(header_name, header_value);
                }
            }
        }
        headermap
    }};
}



pub struct Response {
    pub status: u16,
    pub body: Value,
    pub cookie:  Option<Vec<String>>,
}


pub async fn create_request(uri: &str, mut data: Value, option: RequestOption) -> Result<Response, Value> {
    let mut headers = option.headers.unwrap_or_default();
    let ip = option.real_ip.unwrap_or_else(|| option.ip.unwrap_or_default());
    if !ip.is_empty() {
        headers.insert("X-Real-IP", ip.parse().map_err(|e| format!("Invalid IP: {}", e))?);
        headers.insert("X-Forwarded-For", ip.parse().map_err(|e| format!("Invalid IP: {}", e))?);
    }

    let cookie: Value = option.cookie.unwrap_or_else(|| Value::Object(serde_json::Map::new()));
    headers.insert(COOKIE, get_cookie_string(&uri, &cookie)?.parse().map_err(|e| format!("Invalid Cookie: {}", e))?);

    let mut url;
    let encrypt_data;
    let mut crypto = option.crypto.as_deref().unwrap_or_default();
    let csrf_token = cookie.get("__csrf").and_then(|v| v.as_str()).unwrap_or_default();

    if crypto.is_empty() {
        if CONFIG.get("encrypt").and_then(|v| v.as_bool()).unwrap_or_default() {
            crypto = "eapi";
        } else {
            crypto = "api";
        }
    }

    let domain = CONFIG.get("domain").and_then(|v| v.as_str()).unwrap_or_default();

    match crypto {
        "weapi" => {
            headers.insert(REFERER, domain.parse().map_err(|e| format!("Invalid domain: {}", e))?);
            let ua = option.ua.unwrap_or_else(|| choose_user_agent(crypto, "pc"));
            headers.insert(USER_AGENT, ua.parse().map_err(|e| format!("Invalid User-Agent: {}", e))?);
            data.as_object_mut().unwrap().insert("csrf_token".to_string(), Value::String(csrf_token.to_string()));
            encrypt_data = weapi(&data).map_err(|e| format!("Encryption failed: {}", e))?;
            url = format!("{}/weapi/{}", domain, &uri[5..]);
        }
        "linuxapi" => {
            let ua = option.ua.unwrap_or_else(|| choose_user_agent(crypto, "linux"));
            headers.insert(USER_AGENT, ua.parse().map_err(|e| format!("Invalid User-Agent: {}", e))?);
            let unencrypted_data = json!({
                "method": "POST",
                "url": format!("{}{}", domain, uri),
                "params": data,
            });
            encrypt_data = eapi(&uri, &unencrypted_data).map_err(|e| format!("Encryption failed: {}", e))?;
            url = format!("{}/api/linux/forward", domain);
        }
        "eapi" | "api" => {
            let header = build_headers_from_cookie(&cookie, csrf_token)?;
            let cookie_header = map_to_cookie_header(&header);
            headers.insert(COOKIE, cookie_header.parse().map_err(|e| format!("Invalid Cookie Header: {}", e))?);
            let ua = option.ua.unwrap_or_else(|| choose_user_agent("api", "iphone"));
            headers.insert(USER_AGENT, ua.parse().map_err(|e| format!("Invalid User-Agent: {}", e))?);
            if crypto == "eapi" {
                data.as_object_mut().unwrap().insert("header".to_string(), Value::Object(header.clone()));
                let e_r_temp = option.e_r.unwrap_or_else(|| {
                    data.get("e_r")
                        .and_then(|v| v.as_bool())
                        .unwrap_or_else(|| CONFIG.get("encryptResponse").and_then(|v| v.as_bool()).unwrap_or_default())
                });
                data.as_object_mut().unwrap().insert("e_r".to_string(), Value::Bool(e_r_temp));
                encrypt_data = eapi(&uri, &data).map_err(|e| format!("Encryption failed: {}", e))?;
                url = format!("{}/eapi/{}", domain, &uri[5..]);
            } else {
                url = format!("{}{}", domain, uri);
                encrypt_data = data;
            }
        }
        _ => return Err(error!(format!("Unsupported crypto type: {}", crypto))),
    }


    let mut answer = Response { status: 500, body: json!({}), cookie: None };
    let client_builder = ClientBuilder::new()
        .timeout(Duration::from_secs(10)) // 请求超时
        .connect_timeout(Duration::from_secs(5)) // 连接超时
        .pool_max_idle_per_host(30)
        .pool_idle_timeout(Duration::from_secs(90)) // 设置连接池的空闲超时时间
        .gzip(true); // 启用 Gzip 压缩

    let client_builder = if let Some(proxy_url) = &option.proxy {
        client_builder.proxy(Proxy::all(proxy_url).map_err(|e| format!("Invalid proxy: {}", e))?)
    } else {
        client_builder
    };
    let client = client_builder.build().map_err(|e| format!("Client build error: {}", e))?;
    url = url.add("?").add(&json_to_urlencoded(&encrypt_data));
    let response = client
        .post(&url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| format!("Request error: {}", e))?;

    answer.status = response.status().as_u16();

    let set_cookie_header = response.headers().get_all(SET_COOKIE);
    let set_cookie_string = set_cookie_header
        .iter()
        .map(|header_value| {
            let cookie = header_value.to_str().unwrap_or("").to_string();
            cookie
                .split(';')
                .filter(|part| !part.trim().starts_with("Domain="))
                .collect::<Vec<_>>()
                .join("; ")
        })
        .collect::<Vec<_>>()
        .join("; ");

    // 解析 JSON 响应体
    let body = response
        .json::<Value>()
        .await
        .map_err(|e| format!("Failed to parse response body: {}", e))?;

    // 如果存在 `Set-Cookie`，将其插入到响应体中
    if !set_cookie_string.is_empty() {
        answer.cookie = Some(set_cookie_string.split("; ").map(|s| s.to_string()).collect());
    }
    answer.body = body;
    // 返回增强后的响应体
    Ok(answer)
}

fn get_cookie_string(uri: &str, cookie: &Value) -> Result<String, String> {
    let _ntes_nuid = generate_random_hex_string(32);

    // 获取 OS 信息
    let os = match cookie.get("os") {
        None => choose_os("iphone"),
        Some(os_val) => choose_os(os_val.as_str().unwrap_or("iphone")),
    };

    // 检查 cookie 是否为 JSON 对象
    let mut cookie_temp = cookie
        .as_object()
        .cloned()
        .ok_or_else(|| "Invalid cookie: expected a JSON object".to_string())?;

    // 辅助函数：插入或更新键值
    let insert_or_update = |entry: &mut Map<String, Value>, key: &str, default: &str| {
        entry.insert(
            key.to_string(),
            Value::String(
                cookie
                    .get(key)
                    .and_then(|v| v.as_str())
                    .unwrap_or(default)
                    .to_string(),
            ),
        );
    };

    // 插入/更新必要的 cookie 键值对
    cookie_temp.insert("__remember_me".to_string(), Value::String("true".to_string()));
    cookie_temp.insert("ntes_kaola_ad".to_string(), Value::String("1".to_string()));
    insert_or_update(&mut cookie_temp, "_ntes_nuid", &_ntes_nuid);
    insert_or_update(
        &mut cookie_temp,
        "_ntes_nnid",
        &format!("{},{}", _ntes_nuid, Utc::now().timestamp_millis()),
    );
    insert_or_update(&mut cookie_temp, "wnmcid", &wnmcid());
    insert_or_update(&mut cookie_temp, "WEVNSM", "1.0.0");
    insert_or_update(&mut cookie_temp, "osver", &os.osver);
    insert_or_update(&mut cookie_temp, "deviceId", &DEVICE_ID);
    insert_or_update(&mut cookie_temp, "os", &os.os);
    insert_or_update(&mut cookie_temp, "channel", &os.channel);
    insert_or_update(&mut cookie_temp, "appver", &os.appver);

    // 如果 URI 不包含 "login"，则添加 "NMTID"
    if !uri.contains("login") {
        cookie_temp.insert("NMTID".to_string(), Value::String(generate_random_hex_string(16)));
    }

    // 如果 cookie 中没有 "MUSIC_U"，则设置 "MUSIC_A" 为匿名 token
    if !cookie_temp.contains_key("MUSIC_U") {
        cookie_temp.insert("MUSIC_A".to_string(), Value::String(ANONYMOUS_TOKEN.clone()));
    }

    // 将 cookie 对象转换为字符串
    Ok(cookie_obj_to_string(cookie_temp))
}


fn build_headers_from_cookie(cookie: &Value, csrf_token: &str) -> Result<Map<String, Value>, String> {
    let mut header = Map::new();

    // 确保 `cookie` 是一个 JSON 对象
    let cookie_obj = cookie
        .as_object()
        .ok_or_else(|| "Cookie is not a valid JSON object".to_string())?;

    // 定义字段和默认值
    let fields_with_defaults = [
        ("osver", Value::Null),
        ("deviceId", Value::Null),
        ("os", Value::Null),
        ("appver", Value::String("140".to_string())),
        ("versioncode", Value::String("140".to_string())),
        ("mobilename", Value::Null),
        (
            "buildver",
            Value::String(
                std::time::SystemTime::now()
                    .duration_since(std::time::UNIX_EPOCH)
                    .unwrap()
                    .as_secs()
                    .to_string(),
            ),
        ),
        ("resolution", Value::String("1920x1080".to_string())),
        ("channel", Value::Null),
    ];

    // 插入默认字段
    for (key, default) in fields_with_defaults.iter() {
        header.insert(
            key.to_string(),
            cookie_obj.get(*key).cloned().unwrap_or_else(|| default.clone()),
        );
    }

    header.insert("__csrf".to_string(), Value::String(csrf_token.to_string()));

    // 动态生成 `requestId`
    header.insert(
        "requestId".to_string(),
        Value::String(format!(
            "{}_{}",
            std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap()
                .as_secs(),
            rand::random::<u32>() % 1000
        )),
    );

    // 可选字段 `MUSIC_U` 和 `MUSIC_A`
    if let Some(music_u) = cookie_obj.get("MUSIC_U") {
        header.insert("MUSIC_U".to_string(), music_u.clone());
    }
    if let Some(music_a) = cookie_obj.get("MUSIC_A") {
        header.insert("MUSIC_A".to_string(), music_a.clone());
    }

    Ok(header)
}



