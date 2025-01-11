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

// // 精选电台
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/dj/recommend").route(web::get().to(dj_recommend)));
}

// 入参
define_request_struct!(DjRecommend, {
});

impl DjRecommend {
  async fn requests(req: HttpRequest, query: Query<DjRecommend>) -> Result<Response, Value> {
    let data = json!({});
    create_request(
      "/api/djradio/recommend/v1",
      data,
      create_request_option(extract_headers!(req), &query.common, "weapi"),
    ).await
  }
}
cache_handler!(dj_recommend, DjRecommend);



// // 精选电台
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(`/api/djradio/recommend/v1`, {}, createOption(query, 'weapi'))
// }
