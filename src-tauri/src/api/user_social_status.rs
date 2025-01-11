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

// 用户状态
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/social/user/status").route(web::get().to(user_social_status)));
}

// 入参
define_request_struct!(UserSocialStatus, {
    uid: String,
});

impl UserSocialStatus {
    async fn requests(req: HttpRequest, query: Query<UserSocialStatus>) -> Result<Response, Value> {
        let data = json!({
            "visitorId": query.uid,
        });
        create_request(
            "/api/social/user/status",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        )
        .await
    }
}

cache_handler!(user_social_status, UserSocialStatus);

// // 用户状态
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(
//     `/api/social/user/status`,
//     {
//       visitorId: query.uid,
//     },
//     createOption(query),
//   )
// }
