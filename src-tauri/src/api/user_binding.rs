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

// 用户绑定
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/bindings/{uid}").route(web::get().to(user_binding)));
}

// 入参
define_request_struct!(UserBinding, {
    uid: String,
});

impl UserBinding {
    async fn requests(req: HttpRequest, query: Query<UserBinding>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            &format!("/api/v1/user/bindings/{}", query.uid),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(user_binding, UserBinding);



// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(
//     `/api/v1/user/bindings/${query.uid}`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }