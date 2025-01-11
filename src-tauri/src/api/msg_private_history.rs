
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

// 私信内容
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/msg/private/history").route(web::get().to(msg_private_history)));
}

// 入参
define_request_struct!(MsgPrivateHistory, {
    userId: String,
    limit: Option<i32>,
    time: Option<i64>,
});

impl MsgPrivateHistory {
    async fn requests(req: HttpRequest, query: Query<MsgPrivateHistory>) -> Result<Response, Value> {
        let data = json!({
            "userId": query.userId,
            "limit": query.limit.unwrap_or(30),
            "time": query.time.unwrap_or(0),
            "total": "true",
        });
        create_request(
            "/api/msg/private/history",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(msg_private_history, MsgPrivateHistory);


// // 私信内容
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     userId: query.uid,
//     limit: query.limit || 30,
//     time: query.before || 0,
//     total: 'true',
//   }
//   return request(`/api/msg/private/history`, data, createOption(query, 'weapi'))
// }