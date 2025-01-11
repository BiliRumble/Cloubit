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

// 更换手机
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/replaceCellphone").route(web::get().to(rebind)));
}

// 入参
define_request_struct!(Rebind, {
    captcha: String,
    phone: String,
    oldcaptcha: String,
    ctcode: Option<String>,
});

impl Rebind {
    async fn requests(req: HttpRequest, query: Query<Rebind>) -> Result<Response, Value> {
        let data = json!({
            "captcha": query.captcha,
            "phone": query.phone,
            "oldcaptcha": query.oldcaptcha,
            "ctcode": query.ctcode.clone().unwrap_or_else(|| "86".to_string()),
        });
        create_request(
            "/api/user/replaceCellphone",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(rebind, Rebind);


// // 更换手机
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     captcha: query.captcha,
//     phone: query.phone,
//     oldcaptcha: query.oldcaptcha,
//     ctcode: query.ctcode || '86',
//   }
//   return request(
//     `/api/user/replaceCellphone`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }