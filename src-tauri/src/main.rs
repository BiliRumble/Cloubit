// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

extern crate core;

use crate::api::register_anonimous::register_anonimous;
use crate::util::text::cookie_string_to_json;
use actix_cors::Cors;
use actix_web::http::{header, StatusCode};
use actix_web::middleware::{ErrorHandlerResponse, ErrorHandlers, Logger};
use actix_web::{dev, web, App, HttpResponse, HttpServer, Responder, Result};
use cached::{TimedCache};
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

mod api;
mod util;
use crate::api::router_config::configure_routes;
use crate::util::cache::AppState;

lazy_static! {
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

    pub static ref ANONYMOUS_TOKEN:String = {
        // 从db中查找
        if let Some(anonymous_token) = db.get("anonymousToken").expect("Failed to get anonymousToken") {
            return String::from_utf8(anonymous_token.to_vec()).expect("Failed to convert anonymousToken to string");
        }
        String::new()
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
    let random_index = rand::thread_rng().gen_range(0..lines.len());
    Ok(lines[random_index].clone())
}

async fn init_config() -> sled::Result<()> {
    // 调用 register_anonimous
    let response = register_anonimous().await.map_err(|err| {
        eprintln!("Error during anonymous registration: {}", err);
        sled::Error::Unsupported("Failed to register anonymous user".into())
    })?;

    // 提取并解析 cookie

    let cookie = response
        .body
        .get("cookie")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            eprintln!("Failed to get cookie from response.");
            sled::Error::Unsupported("Missing cookie".into())
        })?;

    // 转换为 JSON 并获取 "MUSIC_A"
    let cookie_obj = cookie_string_to_json(cookie);
    let music_a = cookie_obj
        .get("MUSIC_A")
        .and_then(|v| v.as_str())
        .ok_or_else(|| {
            eprintln!("Failed to extract music_a from cookie.");
            sled::Error::Unsupported("Missing music_a in cookie".into())
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

// 错误处理中间件，处理 404 错误
async fn not_found() -> impl Responder {
    HttpResponse::build(StatusCode::NOT_FOUND).body("404 Not Found")
}

fn add_error_header<B>(mut res: dev::ServiceResponse<B>) -> Result<ErrorHandlerResponse<B>> {
    res.response_mut().headers_mut().insert(
        header::CONTENT_TYPE,
        header::HeaderValue::from_static("Error"),
    );

    Ok(ErrorHandlerResponse::Response(res.map_into_left_body()))
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // 初始化日志系统
    std::env::set_var("RUST_LOG", "info");
    env_logger::init();
    // 初始化缓存，容量为 100 条记录，设置每条记录的过期时间为 300 秒
    let cache = Arc::new(Mutex::new(TimedCache::with_lifespan_and_capacity(300, 100)));
    let app_state = AppState {
        cache: cache.clone(),
    };

    // 服务绑定地址
    let address = "127.0.0.1:8080";

    println!("Starting server at: http://{}", address);
    let server = HttpServer::new(move || {
        App::new()
            // 添加缓存中间件
            .app_data(web::Data::new(app_state.clone()))
            // 添加日志中间件，默认会记录请求和响应
            .wrap(Logger::default())
			// 配置CORS中间件
			.wrap(Cors::default().allow_any_origin().allow_any_method().allow_any_header().supports_credentials())
            // 配置路由
            .configure(configure_routes)
            // 捕获所有未定义的路由
            .default_service(web::route().to(not_found))
            // 配置错误处理中间件
            .wrap(ErrorHandlers::new().handler(StatusCode::INTERNAL_SERVER_ERROR, add_error_header))
    })
    .bind(address)?
    .run();

    // 使用tokio在后台运行服务器
    tokio::spawn(async move {
        if let Err(e) = server.await {
            eprintln!("Server error: {}", e);
        }
    });

    // 在这里启动你的Tauri应用
    qtmusic_lib::run();

    Ok(())
}
