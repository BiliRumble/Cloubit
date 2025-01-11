use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::util::text::to_boolean;
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use web::Query;

// // 电台节目列表
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/program").route(web::get().to(dj_program)));
}

// 入参
define_request_struct!(DjProgram, {
    rid: String,
    asc: String,
    limit: Option<i32>,
    offset: Option<i32>,
});

impl DjProgram {
    async fn requests(req: HttpRequest, query: Query<DjProgram>) -> Result<Response, Value> {
        let data = json!({
        "radioId": query.rid,
        "limit": query.limit.unwrap_or(30),
        "offset": query.offset.unwrap_or(0),
        "asc": to_boolean(query.asc.as_str()),
        });
        create_request(
            "/api/dj/program/byradio",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_program, DjProgram);


// // 电台节目列表
// const { toBoolean } = require('../util')
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     radioId: query.rid,
//     limit: query.limit || 30,
//     offset: query.offset || 0,
//     asc: toBoolean(query.asc),
//   }
//   return request(`/api/dj/program/byradio`, data, createOption(query, 'weapi'))
// }
