
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

// 通知
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/msg/notices").route(web::get().to(msg_notices)));
}

// 入参
define_request_struct!(MsgNotices, {
    limit: Option<i32>,
    time: Option<i64>,
});

impl MsgNotices {
    async fn requests(req: HttpRequest, query: Query<MsgNotices>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.unwrap_or(30),
            "time": query.time.unwrap_or(-1),
        });
        create_request(
            "/api/msg/notices",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(msg_notices, MsgNotices);


// // 通知
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 30,
//     time: query.lasttime || -1,
//   }
//   return request(`/api/msg/notices`, data, createOption(query, 'weapi'))
// }