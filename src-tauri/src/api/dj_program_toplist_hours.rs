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

// // 电台24小时节目榜
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/program/toplist/hours").route(web::get().to(dj_program_toplist_hours)));
}

// 入参
define_request_struct!(DjProgramToplistHours, {
    limit: Option<i32>,
});

impl DjProgramToplistHours {
    async fn requests(req: HttpRequest, query: Query<DjProgramToplistHours>) -> Result<Response, Value> {
        let data = json!({
        "limit": query.limit.unwrap_or(100),
        });
        create_request(
            "/api/djprogram/toplist/hours",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_program_toplist_hours, DjProgramToplistHours);


// // 电台24小时节目榜
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 100,
//     // 不支持 offset
//   }
//   return request(
//     `/api/djprogram/toplist/hours`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
