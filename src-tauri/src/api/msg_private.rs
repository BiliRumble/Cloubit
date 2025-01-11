
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

// 私信
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/msg/private").route(web::get().to(msg_private)));
}

// 入参
define_request_struct!(MsgPrivate, {
    offset: Option<i32>,
    limit: Option<i32>,
});

impl MsgPrivate {
    async fn requests(req: HttpRequest, query: Query<MsgPrivate>) -> Result<Response, Value> {
        let data = json!({
            "offset": query.offset.unwrap_or(0),
            "limit": query.limit.unwrap_or(30),
            "total": "true",
        });
        create_request(
            "/api/msg/private/users",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(msg_private, MsgPrivate);


// // 私信
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     offset: query.offset || 0,
//     limit: query.limit || 30,
//     total: 'true',
//   }
//   return request(`/api/msg/private/users`, data, createOption(query, 'weapi'))
// }