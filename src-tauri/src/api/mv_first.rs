
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

// 最新MV
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/mv/first").route(web::get().to(mv_first)));
}

// 入参
define_request_struct!(MvFirst, {
    area: Option<String>,
    limit: Option<u32>,
});

impl MvFirst {
    async fn requests(req: HttpRequest, query: Query<MvFirst>) -> Result<Response, Value> {
        let data = json!({
            "area": query.area.clone().unwrap_or_default(),
            "limit": query.limit.unwrap_or(30),
            "total": true,
        });
        create_request(
            "/api/mv/first",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(mv_first, MvFirst);


// // 最新MV
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     // 'offset': query.offset || 0,
//     area: query.area || '',
//     limit: query.limit || 30,
//     total: true,
//   }
//   return request(`/api/mv/first`, data, createOption(query))
// }