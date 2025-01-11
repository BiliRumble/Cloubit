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

// 曲风详情
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/style/detail").route(web::get().to(style_detail)));
}

// 入参
define_request_struct!(StyleDetail, {
    tag_id: String,
});

impl StyleDetail {
    async fn requests(req: HttpRequest, query: Query<StyleDetail>) -> Result<Response, Value> {
        let data = json!({
            "tagId": query.tag_id,
        });
        create_request(
            "/api/style-tag/home/head",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(style_detail, StyleDetail);



// // 曲风详情
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     tagId: query.tagId,
//   }
//   return request(`/api/style-tag/home/head`, data, createOption(query, 'weapi'))
// }