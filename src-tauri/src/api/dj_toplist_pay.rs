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

// // 付费精品
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/toplist/pay").route(web::get().to(dj_toplist_pay)));
}

// 入参
define_request_struct!(DjToplistPay, {
    limit: Option<i64>,
});

impl DjToplistPay {
    async fn requests(req: HttpRequest, query: Query<DjToplistPay>) -> Result<Response, Value> {
        let data = json!({
        "limit": query.limit.unwrap_or(100),
    });
        create_request(
            "/api/djradio/toplist/pay",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_toplist_pay, DjToplistPay);


// // 付费精品
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 100,
//     // 不支持 offset
//   }
//   return request(`/api/djradio/toplist/pay`, data, createOption(query, 'weapi'))
// }
