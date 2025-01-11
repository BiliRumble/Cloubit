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

// // 听歌足迹 - 今日收听
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/listen/data/total").route(web::get().to(listen_data_total)));
}

// 入参
define_request_struct!(ListenDataTotal, {
    
});

impl ListenDataTotal {
  async fn requests(req: HttpRequest, query: Query<ListenDataTotal>) -> Result<Response, Value> {
    let data = json!({
        
    });
    create_request(
      "/api/content/activity/listen/data/total",
      data,
      create_request_option(extract_headers!(req), &query.common, ""),
    ).await
  }
}
cache_handler!(listen_data_total, ListenDataTotal);



// // 听歌足迹 - 总收听时长
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(
//     `/api/content/activity/listen/data/total`,
//     {},
//     createOption(query),
//   )
// }
