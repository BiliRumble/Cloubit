
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

// 相似用户
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/simi/user").route(web::get().to(simi_user)));
}

// 入参
define_request_struct!(SimiUser, {
    id: String,
    limit: Option<u32>,
    offset: Option<u32>,
});

impl SimiUser {
    async fn requests(req: HttpRequest, query: Query<SimiUser>) -> Result<Response, Value> {
        let data = json!({
            "songid": query.id,
            "limit": query.limit.unwrap_or(50),
            "offset": query.offset.unwrap_or(0),
        });
        create_request(
            "/api/discovery/simiUser",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}


cache_handler!(simi_user, SimiUser);

// // 相似用户
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     songid: query.id,
//     limit: query.limit || 50,
//     offset: query.offset || 0,
//   }
//   return request(`/api/discovery/simiUser`, data, createOption(query, 'weapi'))
// }