
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

// MV详情
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/mv/detail").route(web::get().to(mv_detail)));
}

// 入参
define_request_struct!(MvDetail, {
    mvid: String,
});

impl MvDetail {
    async fn requests(req: HttpRequest, query: Query<MvDetail>) -> Result<Response, Value> {
        let data = json!({
            "id": query.mvid,
        });
        create_request(
            "/api/v1/mv/detail",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(mv_detail, MvDetail);


// // MV详情
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.mvid,
//   }
//   return request(`/api/v1/mv/detail`, data, createOption(query, 'weapi'))
// }