

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
    cfg.service(web::resource("/social/user/status/rcmd").route(web::get().to(user_social_status_rcmd)));
}

define_request_struct!(UserSocialStatusRcmd, {});

impl UserSocialStatusRcmd {
    async fn requests(req: HttpRequest, query: Query<UserSocialStatusRcmd>) -> Result<Response, Value> {
        create_request(
            "/api/social/user/status/rcmd",
            json!({}),
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}



cache_handler!(user_social_status_rcmd, UserSocialStatusRcmd);

// // 用户状态 - 相同状态的用户
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(`/api/social/user/status/rcmd`, {}, createOption(query))
// }