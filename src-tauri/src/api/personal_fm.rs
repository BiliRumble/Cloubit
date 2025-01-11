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

// 私人FM
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/personal_fm").route(web::get().to(personal_fm)));
}

// 入参
define_request_struct!(PersonalFm, {});

impl PersonalFm {
    async fn requests(req: HttpRequest, query: Query<PersonalFm>) -> Result<Response, Value> {
        create_request(
            "/api/v1/radio/get",
            json!({}),
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(personal_fm, PersonalFm);


//// 私人FM
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(`/api/v1/radio/get`, {}, createOption(query, 'weapi'))
// }