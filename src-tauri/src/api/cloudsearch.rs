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

// // 发送验证码
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/cloudsearch").route(web::get().to(cloudsearch)));
}

// 入参
define_request_struct!(Cloudsearch, {
    keywords: String,
    r#type: Option<i32>,
    limit: Option<i32>,
    offset: Option<i32>
});


impl Cloudsearch {
    async fn requests(req: HttpRequest, query: Query<Cloudsearch>) -> Result<Response, Value> {
        let data = json!({
            "s": query.keywords,
            "type": query.r#type.unwrap_or(1),
            "limit": query.limit.unwrap_or(30),
            "offset": query.offset.unwrap_or(0),
            "total": true
    });
        create_request(
            "/api/cloudsearch/pc",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(cloudsearch, Cloudsearch);

// 搜索

// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     s: query.keywords,
//     type: query.type || 1, // 1: 单曲, 10: 专辑, 100: 歌手, 1000: 歌单, 1002: 用户, 1004: MV, 1006: 歌词, 1009: 电台, 1014: 视频
//     limit: query.limit || 30,
//     offset: query.offset || 0,
//     total: true,
//   }
//   return request(`/api/cloudsearch/pc`, data, createOption(query))
// }
