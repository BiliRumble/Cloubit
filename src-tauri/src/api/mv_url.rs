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


// MV链接
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/mv_url").route(web::get().to(mv_url)));
}

// 入参
define_request_struct!(MvUrl, {
    id: String,
    r: Option<u32>,
});

impl MvUrl {
    async fn requests(req: HttpRequest, query: Query<MvUrl>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "r": query.r.unwrap_or(1080),
        });
        create_request(
            "/api/song/enhance/play/mv/url",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(mv_url, MvUrl);


// // MV链接
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     r: query.r || 1080,
//   }
//   return request(
//     `/api/song/enhance/play/mv/url`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }