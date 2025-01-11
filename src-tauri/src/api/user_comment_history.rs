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

// 用户评论历史
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/comment/history").route(web::get().to(user_comment_history)));
}

// 入参
define_request_struct!(UserCommentHistory, {
    uid: String,
    limit: Option<u32>,
    time: Option<u64>,
});

impl UserCommentHistory {
    async fn requests(req: HttpRequest, query: Query<UserCommentHistory>) -> Result<Response, Value> {
        let data = json!({
            "compose_reminder": "true",
            "compose_hot_comment": "true",
            "limit": query.limit.unwrap_or(10),
            "user_id": query.uid,
            "time": query.time.unwrap_or(0),
        });
        create_request(
            "/api/comment/user/comment/history",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(user_comment_history, UserCommentHistory);



// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     compose_reminder: 'true',
//     compose_hot_comment: 'true',
//     limit: query.limit || 10,
//     user_id: query.uid,
//     time: query.time || 0,
//   }
//   return request(
//     `/api/comment/user/comment/history`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }