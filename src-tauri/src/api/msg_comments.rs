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


// 评论
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/msg/comments").route(web::get().to(msg_comments)));
}

// 入参
define_request_struct!(MsgComments, {
    before: Option<String>,
    limit: Option<i32>,
    uid: String,
});

impl MsgComments {
    async fn requests(req: HttpRequest, query: Query<MsgComments>) -> Result<Response, Value> {
        let data = json!({
            "beforeTime": query.before.clone().unwrap_or_else(|| "-1".to_string()),
            "limit": query.limit.unwrap_or(30),
            "total": "true",
            "uid": query.uid,
        });
        create_request(
            &format!("/api/v1/user/comments/{}", query.uid),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(msg_comments, MsgComments);



//// 评论
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     beforeTime: query.before || '-1',
//     limit: query.limit || 30,
//     total: 'true',
//     uid: query.uid,
//   }
//
//   return request(
//     `/api/v1/user/comments/${query.uid}`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }