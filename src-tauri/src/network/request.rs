use crate::network::crypto::{eapi, weapi};
use crate::network::os::choose_os;
use crate::network::text::{choose_user_agent, cookie_obj_to_string, generate_random_hex_string, json_to_urlencoded, map_to_cookie_header, wnmcid};
use crate::{error, ANONYMOUS_TOKEN, CONFIG, DEVICE_ID};
use chrono::Utc;
use rand::Rng;
use reqwest::header::{HeaderMap, COOKIE, REFERER, SET_COOKIE, USER_AGENT};
use reqwest::{ClientBuilder, Proxy};
use serde_json::{json, Map, Value};
use std::ops::Add;
use std::time::Duration;

use crate::DEBUG;


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

impl Default for RequestOption {
    fn default() -> Self {
        Self {
            crypto: None,
            ip: None,
            real_ip: None,
            headers: None,
            cookie: None,
            proxy: None,
            e_r: None,
            ua: None,
        }
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

#[derive(Debug, Clone)]
pub struct Response {
    pub status: u16,
    pub body: Value,
    pub cookie: Option<Vec<String>>,
    // 新增字段
    pub headers: HeaderMap,                    // 响应头
    pub request_url: String,                   // 实际请求的URL
    pub request_method: String,                // 请求方法
    pub request_headers: HeaderMap,            // 请求头
    pub request_body: Value,                   // 请求体（加密后）
    pub original_data: Value,                  // 原始请求数据（加密前）
    pub crypto_type: String,                   // 使用的加密类型
    pub content_length: Option<u64>,           // 响应内容长度
    pub content_type: Option<String>,          // 响应内容类型
    pub server_info: Option<String>,           // 服务器信息
    pub request_id: Option<String>,            // 请求ID（用于追踪）
    pub cache_info: CacheInfo,                 // 缓存信息
}

#[derive(Debug, Clone)]
pub struct CacheInfo {
    pub cache_hit: bool,
    pub cache_key: Option<String>,
    pub cache_ttl: Option<Duration>,
}


pub async fn create_request(uri: &str, mut data: Value, option: RequestOption) -> Result<Response, Value> {
    let request_id = uuid::Uuid::new_v4().to_string();

    let mut headers = option.headers.unwrap_or_default();
    let ip = option.real_ip.unwrap_or_else(|| option.ip.unwrap_or_default());
    
    if !ip.is_empty() {
        headers.insert("X-Real-IP", ip.parse().map_err(|e| format!("Invalid IP: {}", e))?);
        headers.insert("X-Forwarded-For", ip.parse().map_err(|e| format!("Invalid IP: {}", e))?);
    }

    let cookie: Value = option.cookie.clone().unwrap_or_else(|| Value::Object(serde_json::Map::new()));
    let cookie_string = get_cookie_string(&uri, &cookie)?;
    headers.insert(COOKIE, cookie_string.parse().map_err(|e| format!("Invalid Cookie: {}", e))?);

    let mut url;
    let encrypt_data;
    let mut crypto = option.crypto.as_deref().unwrap_or_default();
    let csrf_token = cookie.get("__csrf").and_then(|v| v.as_str()).unwrap_or_default();

    if crypto.is_empty() {
        crypto = if CONFIG.get("encrypt").and_then(|v| v.as_bool()).unwrap_or_default() {
            "eapi"
        } else {
            "api"
        };
    }
    let domain = CONFIG.get("domain").and_then(|v| v.as_str()).unwrap_or_default();
    let apidomain = CONFIG.get("apiDomain").and_then(|v| v.as_str()).unwrap_or_default();
    let original_data = data.clone(); // 保存原始数据

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
            // 都应该生成客户端cookie
            let header = build_headers_from_cookie(&cookie, csrf_token)?; // 生成header
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
                url = format!("{}/eapi/{}", apidomain, &uri[5..]);
            } else {
                url = format!("{}{}", apidomain, uri);
                encrypt_data = data;
            }
        }
        _ => return Err(error!(format!("Unsupported crypto type: {}", crypto))),
    }

    let request_headers = headers.clone();
    let client_builder = ClientBuilder::new()
        .timeout(Duration::from_secs(10))
        .connect_timeout(Duration::from_secs(5))
        .pool_max_idle_per_host(30)
        .pool_idle_timeout(Duration::from_secs(90))
        .gzip(true);

    let client_builder = if let Some(proxy_url) = &option.proxy {
        client_builder.proxy(Proxy::all(proxy_url).map_err(|e| format!("Invalid proxy: {}", e))?)
    } else {
        client_builder
    };

    let client = client_builder.build().map_err(|e| format!("Client build error: {}", e))?;
    url = url.add("?").add(&json_to_urlencoded(&encrypt_data));

    // 记录参数
    if *DEBUG {
        println!("[{}] Request {}", request_id, url);
    }
    
    let response = client
        .post(&url)
        .headers(headers)
        .send()
        .await
        .map_err(|e| {
            if *DEBUG {
                println!("[{}] Request failed: {}", request_id, e);
            }
            format!("Request error: {}", e)
        })?;

    let status = response.status().as_u16();
    let response_headers = response.headers().clone();
    let content_length = response.content_length();
    let content_type = response.headers()
        .get("content-type")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());
    let server_info = response.headers()
        .get("server")
        .and_then(|v| v.to_str().ok())
        .map(|s| s.to_string());

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

    let body = response
        .json::<Value>()
        .await
        .map_err(|e| {
            format!("Failed to parse response body: {}", e)
        })?;

    let answer = Response {
        status,
        body,
        cookie: if !set_cookie_string.is_empty() {
            Some(set_cookie_string.split("; ").map(|s| s.to_string()).collect())
        } else {
            None
        },
        headers: response_headers,
        request_url: url,
        request_method: "POST".to_string(),
        request_headers,
        request_body: encrypt_data,
        original_data,
        crypto_type: crypto.to_string(),
        content_length,
        content_type,
        server_info,
        request_id: Some(request_id),
        cache_info: CacheInfo {
            cache_hit: false,
            cache_key: None,
            cache_ttl: None,
        },
    };
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
        .unwrap_or_else(|| serde_json::Map::new()); // 如果为空则创建新的Map

    // 辅助函数：插入或更新键值，确保不为空
    let insert_or_update = |entry: &mut Map<String, Value>, key: &str, default: &str| {
        let value = cookie
            .get(key)
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty()) // 过滤空字符串
            .unwrap_or(default);
        entry.insert(key.to_string(), Value::String(value.to_string()));
    };

    // 插入/更新必要的 cookie 键值对，使用OS信息作为默认值
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
    
    // 使用OS信息填充关键字段
    insert_or_update(&mut cookie_temp, "osver", &os.osver);
    insert_or_update(&mut cookie_temp, "deviceId", &DEVICE_ID);
    insert_or_update(&mut cookie_temp, "os", &os.os);
    insert_or_update(&mut cookie_temp, "channel", &os.channel);
    insert_or_update(&mut cookie_temp, "appver", &os.appver);

    // 如果 URI 不包含 "login"，则添加 "NMTID"
    if !uri.contains("login") {
        cookie_temp.insert("NMTID".to_string(), Value::String(generate_random_hex_string(16)));
    }

    // 处理用户凭据
    if !cookie_temp.contains_key("MUSIC_U") {
        // 如果没有MUSIC_U，使用匿名token
        let anonymous_token = if ANONYMOUS_TOKEN.is_empty() {
            "".to_string()
        } else {
            ANONYMOUS_TOKEN.clone()
        };
        cookie_temp.insert("MUSIC_A".to_string(), Value::String(anonymous_token));
    }

    // 将 cookie 对象转换为字符串
    Ok(cookie_obj_to_string(cookie_temp))
}

fn build_headers_from_cookie(cookie: &Value, csrf_token: &str) -> Result<Map<String, Value>, String> {
    let mut header = Map::new();

    // 获取OS信息作为默认值
    let os = choose_os("iphone");
    
    // 确保 `cookie` 是一个 JSON 对象
    let binding = serde_json::Map::new();
    let cookie_obj = cookie
        .as_object()
        .unwrap_or(&binding);

    // 辅助函数：获取值或使用默认值
    let get_value_or_default = |key: &str, default: &str| -> Value {
        cookie_obj.get(key)
            .and_then(|v| v.as_str())
            .filter(|s| !s.is_empty())
            .map(|s| Value::String(s.to_string()))
            .unwrap_or_else(|| Value::String(default.to_string()))
    };

    // 插入字段，确保都有值
    header.insert("osver".to_string(), get_value_or_default("osver", &os.osver));
    header.insert("deviceId".to_string(), get_value_or_default("deviceId", &DEVICE_ID));
    header.insert("os".to_string(), get_value_or_default("os", &os.os));
    header.insert("appver".to_string(), get_value_or_default("appver", &os.appver));
    header.insert("versioncode".to_string(), get_value_or_default("versioncode", "140"));
    header.insert("mobilename".to_string(), get_value_or_default("mobilename", ""));
    header.insert("channel".to_string(), get_value_or_default("channel", &os.channel));
    header.insert(
        "buildver".to_string(),
        get_value_or_default("buildver", &std::time::SystemTime::now()
            .duration_since(std::time::UNIX_EPOCH)
            .unwrap()
            .as_secs()
            .to_string())
    );
    header.insert("resolution".to_string(), get_value_or_default("resolution", "1920x1080"));

    header.insert("__csrf".to_string(), Value::String(csrf_token.to_string()));

    // 动态生成 `requestId`
    let timestamp = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_millis();
    let random_num = rand::rng().random_range(0..1000);
    let request_id = format!("{}_{:04}", timestamp, random_num);
    header.insert("requestId".to_string(), Value::String(request_id));

    // 可选字段 `MUSIC_U` 和 `MUSIC_A`
    if let Some(music_u) = cookie_obj.get("MUSIC_U") {
        header.insert("MUSIC_U".to_string(), music_u.clone());
    } else if !crate::USER_COOKIE.is_empty() {
        header.insert("MUSIC_U".to_string(), Value::String(crate::USER_COOKIE.clone()));
    }    
    if let Some(music_a) = cookie_obj.get("MUSIC_A") {
        header.insert("MUSIC_A".to_string(), music_a.clone());
    }

    Ok(header)
}
