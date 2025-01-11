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


// 云盘数据
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/cloud").route(web::get().to(user_cloud)));
}

// 入参
define_request_struct!(UserCloud, {
    limit: Option<u32>,
    offset: Option<u32>,
});

impl UserCloud {
    async fn requests(req: HttpRequest, query: Query<UserCloud>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.unwrap_or(30),
            "offset": query.offset.unwrap_or(0),
        });
        create_request(
            "/api/v1/cloud/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(user_cloud, UserCloud);


// // 云盘数据
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 30,
//     offset: query.offset || 0,
//   }
//   return request(`/api/v1/cloud/get`, data, createOption(query, 'weapi'))
// }