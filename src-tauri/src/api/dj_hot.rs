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

// // 热门电台
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/hot").route(web::get().to(dj_hot)));
}

// 入参
define_request_struct!(DjHot, {
    limit: Option<i64>,
    offset: Option<i64>,
});

impl DjHot {
    async fn requests(req: HttpRequest, query: Query<DjHot>) -> Result<Response, Value> {
        let data = json!({
        "limit": query.limit.unwrap_or(30),
        "offset": query.offset.unwrap_or(0),
        });
        create_request(
            "/api/djradio/hot/v1",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_hot, DjHot);


// // 热门电台
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 30,
//     offset: query.offset || 0,
//   }
//   return request(`/api/djradio/hot/v1`, data, createOption(query, 'weapi'))
// }
