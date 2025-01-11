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

// // 电台24小时主播榜
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/toplist/hours").route(web::get().to(dj_toplist_hours)));
}

// 入参
define_request_struct!(DjToplistHours, {
    limit: Option<i32>,
});

impl DjToplistHours {
    async fn requests(req: HttpRequest, query: Query<DjToplistHours>) -> Result<Response, Value> {
        let data = json!({
        "limit": query.limit.unwrap_or(100),
    });
        create_request(
            "/api/dj/toplist/hours",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_toplist_hours, DjToplistHours);


// // 电台24小时主播榜
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 100,
//     // 不支持 offset
//   }
//   return request(`/api/dj/toplist/hours`, data, createOption(query, 'weapi'))
// }
