use actix_web::dev::Transform;
use cached::{Cached, TimedCache};
use serde_json::Value;
use std::sync::{Arc, Mutex};
use actix_web::http::StatusCode;
// use actix_web::web::Query;
// use actix_web::{web,Query,HttpResponse, http::StatusCode};

#[derive(Clone)]
pub struct AppState {
    pub(crate) cache: Arc<Mutex<TimedCache<String, String>>>,
}

// 修改后的通用缓存获取函数
pub async fn get_cached_data(cache_key: &String, cache: Arc<Mutex<TimedCache<String, String>>>) -> Option<Value> {
    // println!("cache_key: {}", cache_key);
    // 查看缓存内容
    // println!("cache: {:?}", cache.lock().unwrap().cache_get(cache_key));
    let result = match cache.lock().unwrap().cache_get(cache_key) {
        Some(cached_body) => serde_json::from_str(&cached_body).unwrap(),
        None => { None }
    };
    result
}

pub fn set_cached_data(cache_key: &String, cache: Arc<Mutex<TimedCache<String, String>>>, data: Value) {
    cache.lock().unwrap().cache_set(cache_key.to_string(), data.to_string());
}


#[macro_export]
macro_rules! cache_handler {
    ($fn_name:ident, $struct_name:ident) => {
        pub async fn $fn_name(
            req: HttpRequest,
            query: Query<$struct_name>,
            cache_data: web::Data<AppState>,
        ) -> impl Responder {
            let cache_key = format!("{}{}", req.path(), req.query_string());
            let cached_data = get_cached_data(&cache_key, cache_data.cache.clone()).await;

            if let Some(item) = cached_data {
                return HttpResponse::Ok().json(item);
            }

            let result = $struct_name::requests(req,query).await;
            match result {
                Ok(res) => {
                    let code = res.body.get("code").unwrap().as_i64().unwrap();
                    if res.status == 200  &&  code == 200{
                        set_cached_data(&cache_key, cache_data.cache.clone(), res.body.clone());
                    }
                    let status = StatusCode::from_str(&code.to_string()).unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
                    HttpResponse::build(status).json(res.body)
                }
                Err(val) => {
                    HttpResponse::Ok().json(val)
                }
            }
        }
    };
}