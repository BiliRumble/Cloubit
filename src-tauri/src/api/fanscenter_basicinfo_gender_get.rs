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

// // 粉丝性别比例
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/fanscenter/basicinfo/gender/get").route(web::get().to(fanscenter_basicinfo_gender_get)));
}

// 入参
define_request_struct!(FanscenterBasicinfoGenderGet, {

});

impl FanscenterBasicinfoGenderGet {
  async fn requests(req: HttpRequest, query: Query<FanscenterBasicinfoGenderGet>) -> Result<Response, Value> {
    let data = json!({});
    create_request(
      "/api/fanscenter/basicinfo/gender/get",
      data,
      create_request_option(extract_headers!(req), &query.common, ""),
    ).await
  }
}
cache_handler!(fanscenter_basicinfo_gender_get, FanscenterBasicinfoGenderGet);




// // 粉丝性别比例
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(
//     `/api/fanscenter/basicinfo/gender/get`,
//     data,
//     createOption(query),
//   )
// }
