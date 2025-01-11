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

// // 垃圾桶
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/fm/trash").route(web::get().to(fm_trash)));
}

// 入参
define_request_struct!(FmTrash, {
    id: String,
    time: Option<i32>,
});

impl FmTrash {
    async fn requests(req: HttpRequest, query: Query<FmTrash>) -> Result<Response, Value> {
        let data = json!({
        "songId": query.id,
        "alg": "RT",
        "time": query.time.unwrap_or(25),
    });
        create_request(
            "/api/radio/trash/add",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(fm_trash, FmTrash);


// // 垃圾桶
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     songId: query.id,
//     alg: 'RT',
//     time: query.time || 25,
//   }
//   return request(`/api/radio/trash/add`, data, createOption(query, 'weapi'))
// }
