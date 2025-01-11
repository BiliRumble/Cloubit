use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use web::Query;



// 专辑内容
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/album").route(web::get().to(album)));
}

// 入参
define_request_struct!(Album, {
    id: String,
});


impl Album {
    async fn requests(req: HttpRequest, query: Query<Album>) -> Result<Response, Value> {
        let data = json!({});

        create_request(
            &format!("/api/v1/album/{}", query.id),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}





// 使用宏生成缓存处理函数
cache_handler!(album, Album);