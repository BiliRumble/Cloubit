
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

// 热门搜索
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/search/hot").route(web::post().to(search_hot)));
}

// 入参
define_request_struct!(SearchHot, {});

impl SearchHot {
    async fn requests(req: HttpRequest, query: Query<SearchHot>) -> Result<Response, Value> {
        let data = json!({
            "type": 1111,
        });
        create_request(
            "/api/search/hot",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(search_hot, SearchHot);


// // 热门搜索
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     type: 1111,
//   }
//   return request(`/api/search/hot`, data, createOption(query))
// }