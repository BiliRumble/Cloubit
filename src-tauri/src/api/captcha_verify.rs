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

// // 发送验证码
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/captcha/verify").route(web::get().to(captcha_verify)));
}

// 入参
define_request_struct!(CaptchaVerify, {
    ctcode: Option<String>,
    phone: String,
    captcha: String,
});


impl CaptchaVerify {
  async fn requests(req: HttpRequest, query: Query<CaptchaVerify>) -> Result<Response, Value> {

    let data = json!({
            "ctcode": query.ctcode.clone().unwrap_or("86".to_string()),
            "cellphone": query.phone,
            "captcha": query.captcha,
        });
    create_request(
      "/api/sms/captcha/verify",
      data,
      create_request_option(extract_headers!(req), &query.common, "weapi"),
    ).await
  }
}
cache_handler!(captcha_verify, CaptchaVerify);



// // 校验验证码
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     ctcode: query.ctcode || '86',
//     cellphone: query.phone,
//     captcha: query.captcha,
//   }
//   return request(`/api/sms/captcha/verify`, data, createOption(query, 'weapi'))
// }


