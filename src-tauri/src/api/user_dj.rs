use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;


// 用户电台节目
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/dj").route(web::get().to(user_dj)));
}

// 入参
define_request_struct!(UserDj, {
    uid: String,
    offset: Option<u32>,
    limit: Option<u32>,
});

impl UserDj {
    async fn requests(req: HttpRequest, query: Query<UserDj>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.unwrap_or(30),
            "offset": query.offset.unwrap_or(0),
        });
        create_request(
            &format!("/api/dj/program/{}", query.uid),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(user_dj, UserDj);


// // 用户电台节目
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 30,
//     offset: query.offset || 0,
//   }
//   return request(
//     `/api/dj/program/${query.uid}`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }