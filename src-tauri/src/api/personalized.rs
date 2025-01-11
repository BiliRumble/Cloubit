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

// 推荐歌单
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/personalized/playlist").route(web::get().to(personalized)));
}

// 入参
define_request_struct!(Personalized, {
    limit: Option<u32>,
});

impl Personalized {
    async fn requests(req: HttpRequest, query: Query<Personalized>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.unwrap_or(30),
            "total": true,
            "n": 1000,
        });
        create_request(
            "/api/personalized/playlist",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(personalized, Personalized);



// // 推荐歌单
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 30,
//     // offset: query.offset || 0,
//     total: true,
//     n: 1000,
//   }
//   return request(
//     `/api/personalized/playlist`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }