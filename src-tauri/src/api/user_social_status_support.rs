

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

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/social/status/support").route(web::get().to(user_social_status_support)));
}

define_request_struct!(UserSocialStatusSupport, {});

impl UserSocialStatusSupport {
    async fn requests(req: HttpRequest, query: Query<UserSocialStatusSupport>) -> Result<Response, Value> {
        create_request(
            "/api/social/user/status/support",
            json!({}),
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}



cache_handler!(user_social_status_support, UserSocialStatusSupport);

// // 用户状态 - 支持设置的状态
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(`/api/social/user/status/support`, {}, createOption(query))
// }