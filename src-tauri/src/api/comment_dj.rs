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

// // 电台评论
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/comment/dj").route(web::get().to(comment_dj)));
}

// 入参
define_request_struct!(CommentDj, {
    id: String,
    limit: Option<i32>,
    offset: Option<i32>,
    before: Option<i32>,
});


impl CommentDj {
    async fn requests(req: HttpRequest, query: Query<CommentDj>) -> Result<Response, Value> {
        let data = json!({
        "rid": query.id,
        "limit": query.limit.unwrap_or(20),
        "offset": query.offset.unwrap_or(0),
        "beforeTime": query.before.unwrap_or(0),
    });
        create_request(
            &format!("/api/v1/resource/comments/A_DJ_1_${}", query.id),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(comment_dj, CommentDj);


// // 电台评论
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     rid: query.id,
//     limit: query.limit || 20,
//     offset: query.offset || 0,
//     beforeTime: query.before || 0,
//   }
//   return request(
//     `/api/v1/resource/comments/A_DJ_1_${query.id}`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
