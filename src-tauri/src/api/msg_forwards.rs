
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

// @我
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/forwards/get").route(web::get().to(msg_forwards)));
}

// 入参
define_request_struct!(MsgForwards, {
    offset: Option<i32>,
    limit: Option<i32>,
});

impl MsgForwards {
    async fn requests(req: HttpRequest, query: Query<MsgForwards>) -> Result<Response, Value> {
        let data = json!({
            "offset": query.offset.unwrap_or(0),
            "limit": query.limit.unwrap_or(30),
            "total": "true",
        });
        create_request(
            "/api/forwards/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(msg_forwards, MsgForwards);


// // @我
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     offset: query.offset || 0,
//     limit: query.limit || 30,
//     total: 'true',
//   }
//   return request(`/api/forwards/get`, data, createOption(query, 'weapi'))
// }