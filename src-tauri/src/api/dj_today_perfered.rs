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

// // 电台今日优选
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/today/perfered").route(web::get().to(dj_today_perfered)));
}

// 入参
define_request_struct!(DjTodayPerfered, {
    page: Option<i32>,
});

impl DjTodayPerfered {
    async fn requests(req: HttpRequest, query: Query<DjTodayPerfered>) -> Result<Response, Value> {
        let data = json!({
        "page": query.page.unwrap_or(0),
    });
        create_request(
            "/api/djradio/home/today/perfered",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_today_perfered, DjTodayPerfered);


// // 电台今日优选
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     page: query.page || 0,
//   }
//   return request(
//     `/api/djradio/home/today/perfered`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
