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

// 已购单曲
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/song/purchased").route(web::get().to(song_purchased)));
}

// 入参
define_request_struct!(SongPurchased, {
    limit: Option<u32>,
    offset: Option<u32>,
});

impl SongPurchased {
    async fn requests(req: HttpRequest, query: Query<SongPurchased>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.unwrap_or(20),
            "offset": query.offset.unwrap_or(0),
        });
        create_request(
            "/api/single/mybought/song/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(song_purchased, SongPurchased);


// // 已购单曲
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 20,
//     offset: query.offset || 0,
//   }
//   return request(
//     `/api/single/mybought/song/list`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }