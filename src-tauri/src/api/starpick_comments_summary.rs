
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

// 云村星评馆 - 简要评论列表
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/starpick/comments/summary").route(web::get().to(starpick_comments_summary)));
}

// 入参
define_request_struct!(StarpickCommentsSummary, {});

impl StarpickCommentsSummary {
    async fn requests(req: HttpRequest, query: Query<StarpickCommentsSummary>) -> Result<Response, Value> {
        let data = json!({
            "cursor": json!({
                "offset": 0,
                "blockCodeOrderList": ["HOMEPAGE_BLOCK_NEW_HOT_COMMENT"],
                "refresh": true,
            }),
        });
        create_request(
            "/api/homepage/block/page",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(starpick_comments_summary, StarpickCommentsSummary);



// // 云村星评馆 - 简要评论列表
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     cursor: JSON.stringify({
//       offset: 0,
//       blockCodeOrderList: ['HOMEPAGE_BLOCK_NEW_HOT_COMMENT'],
//       refresh: true,
//     }),
//   }
//   return request(`/api/homepage/block/page`, data, createOption(query))
// }