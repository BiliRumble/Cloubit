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

// // 电台详情
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/detail").route(web::get().to(dj_detail)));
}

// 入参
define_request_struct!(DjDetail, {
    rid: String,
});

impl DjDetail {
    async fn requests(req: HttpRequest, query: Query<DjDetail>) -> Result<Response, Value> {
        let data = json!({
            "id": query.rid,
        });
        create_request(
            "/api/djradio/v2/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_detail, DjDetail);


// // 电台详情
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.rid,
//   }
//   return request(`/api/djradio/v2/get`, data, createOption(query, 'weapi'))
// }
