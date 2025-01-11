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

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/sign/happy/info").route(web::get().to(sign_happy_info)));
}

define_request_struct!(SignHappyInfo, {});

impl SignHappyInfo {
    async fn requests(req: HttpRequest, query: Query<SignHappyInfo>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/sign/happy/info",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}

cache_handler!(sign_happy_info, SignHappyInfo);

// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(`/api/sign/happy/info`, data, createOption(query, 'weapi'))
// }
