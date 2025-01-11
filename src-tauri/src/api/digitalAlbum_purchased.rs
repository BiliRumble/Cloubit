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

// // 我的数字专辑
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/digitalAlbum/purchased").route(web::get().to(digitalAlbum_purchased)));
}

// 入参
define_request_struct!(DigitalAlbumPurchased, {
    limit: Option<i32>,
    offset: Option<i32>,
});

impl DigitalAlbumPurchased {
    async fn requests(req: HttpRequest, query: Query<DigitalAlbumPurchased>) -> Result<Response, Value> {
        let data = json!({
        "limit": query.limit.unwrap_or(30),
        "offset": query.offset.unwrap_or(0),
        "total": true,
    });
        create_request(
            "/api/digitalAlbum/purchased",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(digitalAlbum_purchased, DigitalAlbumPurchased);


// // 我的数字专辑
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 30,
//     offset: query.offset || 0,
//     total: true,
//   }
//   return request(
//     `/api/digitalAlbum/purchased`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
