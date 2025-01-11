
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

// 操作记录
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/feedback/weblog").route(web::get().to(weblog)));
}

// 入参
define_request_struct!(Weblog, {
  data: Option<Value>
});


impl Weblog {
    async fn requests(req: HttpRequest, query: Query<Weblog>) -> Result<Response, Value> {
        let data = query.data.clone().unwrap_or(json!({}));
        create_request(
            "/api/feedback/weblog",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(weblog, Weblog);


// // 操作记录
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(
//     `/api/feedback/weblog`,
//     query.data || {},
//     createOption(query, 'weapi'),
//   )
// }