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

// // 发送与删除评论
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/comment").route(web::get().to(comment)));
}

// 入参
define_request_struct!(Comment, {
    t: i32,
    r#type: String,
    id: String,
    threadId: Option<String>,
    content: Option<String>,
    commentId: Option<String>,
});


impl Comment {
    async fn requests(req: HttpRequest, query: Query<Comment>) -> Result<Response, Value> {
        // 匹配操作类型
        let t = match query.t {
            1 => "add",
            0 => "delete",
            2 => "reply",
            _ => "",
        };

        // 获取资源类型
        let resource_type = RESOURCE_TYPE_MAP.get(&query.r#type).unwrap_or(&"".to_string()).to_string();

        // 根据类型构造 threadId
        let mut data = json!({
            "threadId": if resource_type == "A_EV_2_" {
                query.threadId.clone().unwrap_or_default()
            } else {
                format!("{}{}", resource_type, query.id)
            }
        });

        // 根据不同操作类型，动态添加字段
        match t {
            "add" => {
                if let Some(content) = &query.content {
                    data["content"] = json!(content);
                }
            }
            "delete" => {
                if let Some(comment_id) = &query.commentId {
                    data["commentId"] = json!(comment_id);
                }
            }
            "reply" => {
                if let Some(comment_id) = &query.commentId {
                    data["commentId"] = json!(comment_id);
                }
                if let Some(content) = &query.content {
                    data["content"] = json!(content);
                }
            }
            _ => {}
        }

        // 发起请求
        create_request(
            &format!("/api/resource/comments/{}", t),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(comment, Comment);


// const { resourceTypeMap } = require('../util/config.json')
// // 发送与删除评论
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.t = {
//     1: 'add',
//     0: 'delete',
//     2: 'reply',
//   }[query.t]
//   query.type = resourceTypeMap[query.type]
//   const data = {
//     threadId: query.type + query.id,
//   }
// 
//   if (query.type == 'A_EV_2_') {
//     data.threadId = query.threadId
//   }
//   if (query.t == 'add') data.content = query.content
//   else if (query.t == 'delete') data.commentId = query.commentId
//   else if (query.t == 'reply') {
//     data.commentId = query.commentId
//     data.content = query.content
//   }
//   return request(
//     `/api/resource/comments/${query.t}`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
