use tauri::Manager;

use crate::network::register::register_anonimous;
use crate::network::text::cookie_string_to_json;
use cached::TimedCache;
use lazy_static::lazy_static;
use rand::Rng;
use serde_json::{from_str, Value};
use sled::Db;
use std::collections::HashMap;
use std::fs::File;
use std::io;
use std::io::{BufRead, BufReader};
use std::sync::{Arc, LazyLock, Mutex};
use std::env;

mod utils;
mod network;
mod services;
mod manager;
mod core;
mod interface;

use crate::network::request::{create_request, RequestOption, Response};

use interface::{api_search, api_get_song_url, api_login, api_login_status, api_logout};

#[macro_export]
macro_rules! error {
    ($msg:expr) => {
        serde_json::json!({
            "error": $msg,
            "code": 500
        })
    };
}

pub struct NeteaseMusicApi {
    cache: Arc<Mutex<TimedCache<String, String>>>,
}

impl NeteaseMusicApi {
    pub fn new() -> Self {
        let cache = Arc::new(Mutex::new(TimedCache::with_lifespan_and_capacity(300, 100)));
        Self { cache }
    }

    pub async fn request(&self, uri: &str, data: Value, option: RequestOption) -> Result<Response, Value> {
        create_request(uri, data, option).await
    }

    pub fn get_cache(&self) -> Arc<Mutex<TimedCache<String, String>>> {
        self.cache.clone()
    }
}

// 全局变量
lazy_static! {
    pub static ref NETEASE_API: NeteaseMusicApi = NeteaseMusicApi::new();
    
    pub static ref RESOURCE_TYPE_MAP: LazyLock<HashMap<String, String>> = LazyLock::new(|| {
        // 读取config - TODO: 使用tauri2 resources
        let config_str = std::fs::read_to_string("config.json").expect("Failed to read config.json");
        let config_value: Value = from_str(&config_str).expect("Failed to parse config.json");
        config_value
            .get("resourceTypeMap")
            .expect("Failed to get resourceTypeMap")
            .as_object()
            .expect("resourceTypeMap should be an object")
            .iter()
            .map(|(k, v)| (k.clone(), v.as_str().unwrap_or("").to_string()))
            .collect()
    });

    pub static ref CONFIG: Value = {
        // 读取config - TODO: 使用tauri2 resources
        let config_str = std::fs::read_to_string("config.json").expect("Failed to read config.json");
        let config_value: Value = from_str(&config_str).expect("Failed to parse config.json");
        config_value.get("APP_CONF")
            .expect("Failed to get APP_CONF")
            .clone()
    };

    pub static ref db:Db = {
        let temp_dir = env::temp_dir();
        let db_path = temp_dir.join("temp_db");
        sled::open(db_path).expect("Failed to open temp_db")
    };

    pub static ref DEVICE_ID:String = {
        // 从db中查找
        // if let Some(device_id) = db.get("deviceId").expect("Failed to get deviceId") {
        //     return String::from_utf8(device_id.to_vec()).expect("Failed to convert deviceId to string");
        // }
        // 从文件中读取
        let device_id = get_random_device_id("deviceid.txt").expect("Failed to get random device id");
        db.insert("deviceId", device_id.as_str()).expect("Failed to insert deviceId");
        db.flush().expect("Failed to flush db");
        device_id
    };

    pub static ref ANONYMOUS_TOKEN: String = {
        // 从db中查找
        if let Some(anonymous_token) = db.get("anonymousToken").expect("Failed to get anonymousToken") {
            return String::from_utf8(anonymous_token.to_vec()).expect("Failed to convert anonymousToken to string");
        }
        String::new()
    };

    pub static ref USER_COOKIE: String = {
        if let Some(user_token) = db.get("music_u_token").expect("Failed to get user_cookie") {
            return String::from_utf8(user_token.to_vec()).expect("Failed to convert user_cookie to string");
        }
        String::new()
    };

    // debug开关
    pub static ref DEBUG:bool = {
        false
    };
}


fn get_random_device_id(filename: &str) -> io::Result<String> {
    // 打开文件
    let file = File::open(filename)?;
    let reader = BufReader::new(file);

    // 收集所有行到 Vec 中
    let lines: Vec<_> = reader.lines().collect::<Result<_, _>>()?;
    if lines.is_empty() {
        return Err(io::Error::new(io::ErrorKind::InvalidData, "File is empty"));
    }

    // 随机选择一行
    let random_index = rand::rng().random_range(0..lines.len());
    Ok(lines[random_index].clone())
}

async fn init_config() -> sled::Result<()> {
    // 调用 register_anonimous
    let response = register_anonimous().await.map_err(|err| {
        eprintln!("Error during anonymous registration: {}", err);
        sled::Error::Unsupported("Failed to register anonymous user".into())
    })?;

    let cookie_string = if let Some(cookies) = &response.cookie {
        cookies.join("; ")
    } else {
        return Err(sled::Error::Unsupported("Missing cookie in response".into()));
    };

    // 转换为 JSON 并获取 "MUSIC_A"
    let cookie_obj = cookie_string_to_json(&cookie_string);

    let music_a = cookie_obj
        .get("MUSIC_A")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            eprintln!("Failed to extract MUSIC_A from cookie.");
            sled::Error::Unsupported("Missing MUSIC_A in cookie".into())
        })?;

    // 插入到数据库
    db.insert("anonymousToken", music_a).map_err(|err| {
        eprintln!("Failed to insert music_a into database: {}", err);
        err
    })?;

    db.flush().map_err(|err| {
        eprintln!("Failed to flush database: {}", err);
        err
    })?;

    Ok(())
}

pub fn run() {
    // 初始化日志系统
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    
    // 触发全局变量的初始化
    let _ = &*NETEASE_API;
    let _ = &*CONFIG;
    let _ = &*DEVICE_ID;
    let _ = &*USER_COOKIE;
    
    // 异步初始化配置
    tokio::spawn(async {
        if ANONYMOUS_TOKEN.is_empty() {
            if let Err(e) = init_config().await {
                eprintln!("Failed to initialize anonymous token: {}", e);
            }
        }
    });

    tauri::Builder::default()
        .setup(|app| {
            #[cfg(all(desktop))]
            {
                let handle = app.handle();
                core::tray::create_tray(handle)?;
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            api_search,
            api_get_song_url,
            api_login,
            api_login_status,
            api_logout,
        ])
        .plugin(tauri_plugin_single_instance::init(|app, _args, _cwd| {
            let main = app.get_webview_window("main")
                .expect("Could not find main window");
            main.show().expect("Could not show main window");
            main.set_focus().expect("Could not set focus to main window");
        }))
        .plugin(tauri_plugin_global_shortcut::Builder::new().build())
        .plugin(tauri_plugin_window_state::Builder::default().build())
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("Could not run application");
}
