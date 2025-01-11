
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
    cfg.service(web::resource("/user/mutualfollow/get").route(web::get().to(user_mutualfollow_get)));
}

define_request_struct!(UserMutualFollowGet, {
    uid: String,
});

impl UserMutualFollowGet {
    async fn requests(req: HttpRequest, query: Query<UserMutualFollowGet>) -> Result<Response, Value> {
        let data = json!({
            "friendid": query.uid,
        });
        create_request(
            "/api/user/mutualfollow/get",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}

cache_handler!(user_mutualfollow_get, UserMutualFollowGet);

// // 用户是否互相关注
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     friendid: query.uid,
//   }
//   return request(`/api/user/mutualfollow/get`, data, createOption(query))
// }