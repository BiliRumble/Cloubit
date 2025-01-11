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

// // 一起听状态
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/listentogether/status").route(web::get().to(listentogether_status)));
}

// 入参
define_request_struct!(ListentogetherStatus, {

});

impl ListentogetherStatus {
  async fn requests(req: HttpRequest, query: Query<ListentogetherStatus>) -> Result<Response, Value> {
    let data = json!({});
    create_request(
      "/api/listen/together/status/get",
      data,
      create_request_option(extract_headers!(req), &query.common, "weapi"),
    ).await
  }
}
cache_handler!(listentogether_status, ListentogetherStatus);



// // 一起听状态
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(
//     `/api/listen/together/status/get`,
//     {},
//     createOption(query, 'weapi'),
//   )
// }
