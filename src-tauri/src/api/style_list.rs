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


// 曲风列表
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/style/list").route(web::get().to(style_list)));
}

// 入参
define_request_struct!(StyleList, {});

impl StyleList {
    async fn requests(req: HttpRequest, query: Query<StyleList>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/tag/list/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(style_list, StyleList);


// // 曲风列表
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(`/api/tag/list/get`, data, createOption(query, 'weapi'))
// }