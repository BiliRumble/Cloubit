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

// 电台分类列表
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/catelist").route(web::get().to(dj_catelist)));
}

// 入参
define_request_struct!(DjCatelist, {
});

impl DjCatelist {
    async fn requests(req: HttpRequest, query: Query<DjCatelist>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/djradio/category/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_catelist, DjCatelist);


// 电台分类列表

// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(`/api/djradio/category/get`, {}, createOption(query, 'weapi'))
// }
