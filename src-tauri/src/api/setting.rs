
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

// 设置
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/setting").route(web::get().to(setting)));
}

define_request_struct!(Setting, {});

impl Setting {
    async fn requests(req: HttpRequest, query: Query<Setting>) -> Result<Response, Value>{
        let data = json!({});
        create_request(
            "/api/user/setting",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}


cache_handler!(setting, Setting);

// // 设置
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(`/api/user/setting`, data, createOption(query, 'weapi'))
// }