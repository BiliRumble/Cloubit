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

// 检测手机号码是否已注册
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/cellphone/existence/check").route(web::post().to(cellphone_existence_check)));
}

// 入参
define_request_struct!(CellphoneExistenceCheck, {
    phone: String,
    countrycode: Option<String>,
});

impl CellphoneExistenceCheck {
    async fn requests(req: HttpRequest, query: Query<CellphoneExistenceCheck>) -> Result<Response, Value> {
        let data = json!({
            "cellphone": query.phone,
            "countrycode": query.countrycode.clone().unwrap_or_else(|| "".to_string()),
        });
        create_request(
            "/api/cellphone/existence/check",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(cellphone_existence_check, CellphoneExistenceCheck);



// // 检测手机号码是否已注册
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     cellphone: query.phone,
//     countrycode: query.countrycode,
//   }
//   return request(`/api/cellphone/existence/check`, data, createOption(query))
// }