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

// 点赞与取消点赞资源
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/resource/like").route(web::get().to(resource_like)));
}

// 入参
define_request_struct!(ResourceLike, {
    id: String,
    t: String,
    r#type: String,
    threadId: Option<String>,
});

impl ResourceLike {
    async fn requests(req: HttpRequest, query: Query<ResourceLike>) -> Result<Response, Value> {
        let t = if query.t == "1" { "like" } else { "unlike" };
        let resource_type = RESOURCE_TYPE_MAP
            .get(&query.r#type)
            .unwrap_or(&"".to_string())
            .to_string();
        let mut data = json!({
            "threadId": resource_type.clone() + &query.id,
        });
        if resource_type == "A_EV_2_" {
            data["threadId"] = json!(query.threadId.clone().unwrap_or_default());
        }
        create_request(
            &format!("/api/resource/{}", t),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}

cache_handler!(resource_like, ResourceLike);

// // 点赞与取消点赞资源
// const { resourceTypeMap } = require('../util/config.json')
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.t = query.t == 1 ? 'like' : 'unlike'
//   query.type = resourceTypeMap[query.type]
//   const data = {
//     threadId: query.type + query.id,
//   }
//   if (query.type === 'A_EV_2_') {
//     data.threadId = query.threadId
//   }
//   return request(`/api/resource/${query.t}`, data, createOption(query, 'weapi'))
// }
