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

// //热门话题
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/hug/comment").route(web::get().to(hug_comment)));
}

// 入参
define_request_struct!(Hugcomment, {
    uid: String,
    cid: String,
    r#type: String,
    sid: String,
});

impl Hugcomment {
    async fn requests(req: HttpRequest, query: Query<Hugcomment>) -> Result<Response, Value> {
        let resource_type = RESOURCE_TYPE_MAP.get(&query.r#type).unwrap_or(&"".to_string()).to_string();
        let data = json!({
            "targetUserId": query.uid,
            "commentId": query.cid,
            "threadId": resource_type + &query.sid,
        });
        create_request(
            "/api/v2/resource/comments/hug/listener",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(hug_comment, Hugcomment);


// const { resourceTypeMap } = require('../util/config.json')
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.type = resourceTypeMap[query.type || 0]
//   const threadId = query.type + query.sid
//   const data = {
//     targetUserId: query.uid,
//     commentId: query.cid,
//     threadId: threadId,
//   }
//   return request(
//     `/api/v2/resource/comments/hug/listener`,
//     data,
//     createOption(query),
//   )
// }
