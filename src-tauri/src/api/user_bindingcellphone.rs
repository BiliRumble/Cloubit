use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;
use crypto::digest::Digest;
use crypto::md5::Md5;
use std::str::FromStr;

// 绑定手机
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/bindingCellphone").route(web::get().to(user_bindingcellphone)));
}

// 入参
define_request_struct!(UserBindingCellphone, {
    phone: String,
    countrycode: Option<String>,
    captcha: String,
    password: Option<String>,
});

impl UserBindingCellphone {
    async fn requests(req: HttpRequest, query: Query<UserBindingCellphone>) -> Result<Response, Value> {
        let mut hasher = Md5::new();
        let password = if let Some(pwd) = &query.password {
            hasher.input_str(pwd);
            hasher.result_str()
        } else {
            "".to_string()
        };
        let data = json!({
            "phone": query.phone,
            "countrycode": query.countrycode.clone().unwrap_or_else(|| "86".to_string()),
            "captcha": query.captcha,
            "password": password,
        });
        create_request(
            "/api/user/bindingCellphone",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(user_bindingcellphone, UserBindingCellphone);


// const CryptoJS = require('crypto-js')
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     phone: query.phone,
//     countrycode: query.countrycode || '86',
//     captcha: query.captcha,
//     password: query.password ? CryptoJS.MD5(query.password).toString() : '',
//   }
//   return request(
//     `/api/user/bindingCellphone`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }