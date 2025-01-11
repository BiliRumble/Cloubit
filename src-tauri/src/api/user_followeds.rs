
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

// 关注TA的人(粉丝)
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/followeds").route(web::get().to(user_followeds)));
}

// 入参
define_request_struct!(UserFolloweds, {
    uid: String,
    offset: Option<u32>,
    limit: Option<u32>,
});

impl UserFolloweds {
    async fn requests(req: HttpRequest, query: Query<UserFolloweds>) -> Result<Response, Value> {
        let data = json!({
            "userId": query.uid,
            "time": "0",
            "limit": query.limit.unwrap_or(30),
            "offset": query.offset.unwrap_or(0),
            "getcounts": "true",
        });
        create_request(
            &format!("/api/user/getfolloweds/{}", query.uid),
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(user_followeds, UserFolloweds);



// // 关注TA的人(粉丝)
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     userId: query.uid,
//     time: '0',
//     limit: query.limit || 30,
//     offset: query.offset || 0,
//     getcounts: 'true',
//   }
//   return request(
//     `/api/user/getfolloweds/${query.uid}`,
//     data,
//     createOption(query),
//   )
// }