
use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/replaceCellphone").route(web::get().to(user_replacephone)));
}

define_request_struct!(UserReplacePhone, {
    phone: String,
    captcha: String,
    oldcaptcha: String,
    countrycode: Option<String>,
});

impl UserReplacePhone {
    async fn requests(req: HttpRequest, query: Query<UserReplacePhone>) -> Result<Response, Value> {
        let data = json!({
            "phone": query.phone,
            "captcha": query.captcha,
            "oldcaptcha": query.oldcaptcha,
            "countrycode": query.countrycode.clone().unwrap_or_else(|| "86".to_string()),
        });
        create_request(
            "/api/user/replaceCellphone",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}

cache_handler!(user_replacephone, UserReplacePhone);


// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     phone: query.phone,
//     captcha: query.captcha,
//     oldcaptcha: query.oldcaptcha,
//     countrycode: query.countrycode || '86',
//   }
//   return request(
//     `/api/user/replaceCellphone`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }