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

// // 点赞与取消点赞评论
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/comment/like").route(web::get().to(comment_like)));
}

// 入参
define_request_struct!(CommentLike, {
    id: String,
    r#type: String,
    cid: String,
    threadId: String,
    t: String,
});


impl CommentLike {
    async fn requests(req: HttpRequest, query: Query<CommentLike>) -> Result<Response, Value> {
        let resource_type = RESOURCE_TYPE_MAP.get(&query.r#type).unwrap_or(&"".to_string()).to_string();
        let mut data = json!({
        "threadId":format!("{}{}",&resource_type,&query.id),
        "commentId": query.cid,
    });

        if resource_type == "A_EV_2_" {
            data["threadId"] = Value::String(query.threadId.clone());
        };

        create_request(
            &format!("/api/v1/comment/{}", query.t),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(comment_like, CommentLike);

// const { resourceTypeMap } = require('../util/config.json')
// // 点赞与取消点赞评论
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.t = query.t == 1 ? 'like' : 'unlike'
//   query.type = resourceTypeMap[query.type]
//   const data = {
//     threadId: query.type + query.id,
//     commentId: query.cid,
//   }
//   if (query.type == 'A_EV_2_') {
//     data.threadId = query.threadId
//   }
//   return request(
//     `/api/v1/comment/${query.t}`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
