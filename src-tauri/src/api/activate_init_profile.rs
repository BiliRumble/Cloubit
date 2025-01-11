use std::str::FromStr;
use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;

// 初始化名字
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/activate/init/profile").route(web::get().to(activate_init_profile)));
}

// 入参
define_request_struct!(ActivateInitProfile, {
    nickname: String,
});


impl ActivateInitProfile {
    async fn requests(req: HttpRequest, query: Query<ActivateInitProfile>) -> Result<Response, Value> {
        let data = json!({
            "nickname": query.nickname,
        });


        create_request(
            "/api/activate/initProfile",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}





// 使用宏生成缓存处理函数
cache_handler!(activate_init_profile, ActivateInitProfile);