use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use md5;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use web::Query;

// 注册账号
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register/cellphone").route(web::get().to(register_cellphone)));
}

// 入参
define_request_struct!(RegisterCellphone, {
    captcha: String,
    phone: String,
    password: String,
    nickname: String,
    countrycode: Option<String>,
});

impl RegisterCellphone {
    async fn requests(
        req: HttpRequest,
        query: Query<RegisterCellphone>,
    ) -> Result<Response, Value> {
        let data = json!({
            "captcha": query.captcha,
            "phone": query.phone,
            "password": format!("{:x}", md5::compute(query.password.clone())),
            "nickname": query.nickname,
            "countrycode": query.countrycode.clone().unwrap_or_else(|| "86".to_string()),
        });
        create_request(
            "/api/register/cellphone",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}

cache_handler!(register_cellphone, RegisterCellphone);

// // 注册账号
// const CryptoJS = require('crypto-js')
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     captcha: query.captcha,
//     phone: query.phone,
//     password: CryptoJS.MD5(query.password).toString(),
//     nickname: query.nickname,
//     countrycode: query.countrycode || '86',
//   }
//   return request(`/api/register/cellphone`, data, createOption(query, 'weapi'))
// }
