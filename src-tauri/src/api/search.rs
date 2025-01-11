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

// 搜索
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/search").route(web::post().to(search)));
}

// 入参
define_request_struct!(Search, {
    keywords: String,
    r#type: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
});

impl Search {
    async fn requests(req: HttpRequest, query: Query<Search>) -> Result<Response, Value> {
        let data = if query.r#type.as_deref() == Some("2000") {
            json!({
                "keyword": query.keywords,
                "scene": "normal",
                "limit": query.limit.unwrap_or(30),
                "offset": query.offset.unwrap_or(0),
            })
        } else {
            json!({
                "s": query.keywords,
                "type": query.r#type.clone().unwrap_or_else(|| "1".to_string()),
                "limit": query.limit.unwrap_or(30),
                "offset": query.offset.unwrap_or(0),
            })
        };
        let url = if query.r#type.as_deref() == Some("2000") {
            "/api/search/voice/get"
        } else {
            "/api/search/get"
        };
        create_request(
            url,
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(search, Search);


// // 搜索
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   if (query.type && query.type == '2000') {
//     const data = {
//       keyword: query.keywords,
//       scene: 'normal',
//       limit: query.limit || 30,
//       offset: query.offset || 0,
//     }
//     return request(`/api/search/voice/get`, data, createOption(query))
//   }
//   const data = {
//     s: query.keywords,
//     type: query.type || 1, // 1: 单曲, 10: 专辑, 100: 歌手, 1000: 歌单, 1002: 用户, 1004: MV, 1006: 歌词, 1009: 电台, 1014: 视频
//     limit: query.limit || 30,
//     offset: query.offset || 0,
//   }
//   return request(`/api/search/get`, data, createOption(query))
// }