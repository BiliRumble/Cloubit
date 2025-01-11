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

// MV排行榜
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/top/mv").route(web::get().to(top_mv)));
}

// 入参
define_request_struct!(TopMv, {
    area: Option<String>,
    limit: Option<u8>,
    offset: Option<u8>,
});

impl TopMv {
    async fn requests(req: HttpRequest, query: Query<TopMv>) -> Result<Response, Value> {
        let data = json!({
            "area": query.area.clone().unwrap_or("".to_string()),
            "limit": query.limit.unwrap_or(30),
            "offset": query.offset.unwrap_or(0),
            "total": true,
        });
        create_request(
            "/api/mv/toplist",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(top_mv, TopMv);


// // MV排行榜
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     area: query.area || '',
//     limit: query.limit || 30,
//     offset: query.offset || 0,
//     total: true,
//   }
//   return request(`/api/mv/toplist`, data, createOption(query, 'weapi'))
// }