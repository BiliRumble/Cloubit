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

// 收藏的专栏
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/topic/sublist").route(web::get().to(topic_sublist)));
}

// 入参
define_request_struct!(TopicSublist, {
    limit: Option<u8>,
    offset: Option<u8>,
});

impl TopicSublist {
    async fn requests(req: HttpRequest, query: Query<TopicSublist>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.unwrap_or(50),
            "offset": query.offset.unwrap_or(0),
            "total": true,
        });
        create_request(
            "/api/topic/sublist",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(topic_sublist, TopicSublist);


// // 收藏的专栏
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 50,
//     offset: query.offset || 0,
//     total: true,
//   }
//   return request(`/api/topic/sublist`, data, createOption(query, 'weapi'))
// }