
use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

// 用户动态
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/event").route(web::get().to(user_event)));
}

// 入参
define_request_struct!(UserEvent, {
    uid: String,
    lasttime: Option<i64>,
    limit: Option<u32>,
});

impl UserEvent {
    async fn requests(req: HttpRequest, query: Query<UserEvent>) -> Result<Response, Value> {
        let data = json!({
            "getcounts": true,
            "time": query.lasttime.unwrap_or(-1),
            "limit": query.limit.unwrap_or(30),
            "total": false,
        });
        create_request(
            &format!("/api/event/get/{}", query.uid),
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(user_event, UserEvent);


// // 用户动态
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     getcounts: true,
//     time: query.lasttime || -1,
//     limit: query.limit || 30,
//     total: false,
//   }
//   return request(`/api/event/get/${query.uid}`, data, createOption(query))
// }