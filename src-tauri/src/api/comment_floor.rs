use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers, RESOURCE_TYPE_MAP};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use web::Query;

// // 获取动态评论
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/comment/floor").route(web::get().to(comment_floor)));
}

// 入参
define_request_struct!(CommentFloor, {
    parentCommentId: String,
    id: String,
    time: Option<i64>,
    limit: Option<i64>,
    r#type : String,
});


impl CommentFloor {
    async fn requests(req: HttpRequest, query: Query<CommentFloor>) -> Result<Response, Value> {
        let resource_type = RESOURCE_TYPE_MAP.get(&query.r#type).unwrap_or(&"".to_string()).to_string();
        let data = json!({
            "parentCommentId": query.parentCommentId,
            "threadId": resource_type + &query.id,
            "time": query.time.unwrap_or(-1),
            "limit": query.limit.unwrap_or(20),
        });
        create_request(
            "/api/resource/comment/floor/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(comment_floor, CommentFloor);


// const { resourceTypeMap } = require('../util/config.json')
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.type = resourceTypeMap[query.type]
//   const data = {
//     parentCommentId: query.parentCommentId,
//     threadId: query.type + query.id,
//     time: query.time || -1,
//     limit: query.limit || 20,
//   }
//   return request(
//     `/api/resource/comment/floor/get`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
