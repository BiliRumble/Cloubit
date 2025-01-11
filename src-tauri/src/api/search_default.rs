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

// 默认搜索关键词
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/search/default").route(web::get().to(search_default)));
}

// 入参
define_request_struct!(SearchDefault, {

});

impl SearchDefault {
    async fn requests(req: HttpRequest, query: Query<SearchDefault>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/search/defaultkeyword/get",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}


cache_handler!(search_default, SearchDefault);


// // 默认搜索关键词
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(`/api/search/defaultkeyword/get`, {}, createOption(query))
// }