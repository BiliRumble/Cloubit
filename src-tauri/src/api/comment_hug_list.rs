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

// // 热门评论
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/comment/hug/list").route(web::get().to(comment_hug_list)));
}

// 入参
define_request_struct!(CommentHugList, {
    uid: String,
    sid: String,
    cid: String,
    cursor: Option<String>,
    r#type: String,
    page: Option<i64>,
    idCursor: Option<i64>,
    pageSize: Option<i64>
});


impl CommentHugList {
    async fn requests(req: HttpRequest, query: Query<CommentHugList>) -> Result<Response, Value> {
        let resource_type = RESOURCE_TYPE_MAP.get(&query.r#type).unwrap_or(&"".to_string()).to_string();
        let thread_id = format!("{}{}", resource_type, query.sid);
        let data = json!({
            "targetUserId": query.uid,
            "commentId": query.cid,
            "cursor": query.cursor.clone().unwrap_or("-1".to_string()),
            "threadId": thread_id,
            "pageNo": query.page.unwrap_or(1),
            "idCursor": query.idCursor.unwrap_or(-1),
            "pageSize": query.pageSize.unwrap_or(100),
        });
        create_request(
            "/api/v2/resource/comments/hug/list",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(comment_hug_list, CommentHugList);


// const { resourceTypeMap } = require('../util/config.json')
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.type = resourceTypeMap[query.type || 0]
//   const threadId = query.type + query.sid
//   const data = {
//     targetUserId: query.uid,
//     commentId: query.cid,
//     cursor: query.cursor || '-1',
//     threadId: threadId,
//     pageNo: query.page || 1,
//     idCursor: query.idCursor || -1,
//     pageSize: query.pageSize || 100,
//   }
//   return request(
//     `/api/v2/resource/comments/hug/list`,
//     data,
//     createOption(query),
//   )
// }
