use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;
use serde_json::{json, Value};
use web::Query;

// 推荐电台
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/personalized/djprogram").route(web::get().to(personalized_djprogram)));
}

// 入参
define_request_struct!(PersonalizedDjprogram, {});

impl PersonalizedDjprogram {
    async fn requests(req: HttpRequest, query: Query<PersonalizedDjprogram>) -> Result<Response, Value> {
        create_request(
            "/api/personalized/djprogram",
            json!({}),
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(personalized_djprogram, PersonalizedDjprogram);


// // 推荐电台
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(
//     `/api/personalized/djprogram`,
//     {},
//     createOption(query, 'weapi'),
//   )
// }