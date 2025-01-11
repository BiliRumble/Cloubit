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

// // 评论
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/comment/new").route(web::get().to(comment_new)));
}

// 入参
define_request_struct!(CommentNew, {
    r#type: String,
    id: String,
    page_size: Option<u32>,
    page_no: Option<u32>,
    sort_type: Option<u32>,
    cursor: Option<String>,
    show_inner: Option<bool>,
});

impl CommentNew {
    async fn requests(req: HttpRequest, query: Query<CommentNew>) -> Result<Response, Value> {
        let resource_type = RESOURCE_TYPE_MAP.get(&query.r#type).unwrap_or(&"".to_string()).to_string();
        let thread_id = format!("{}{}", resource_type, query.id);

        let page_size = query.page_size.unwrap_or(20);
        let page_no = query.page_no.unwrap_or(1);

        let sort_type = query.sort_type.unwrap_or(99);
        let mut cursor = String::new();

        match sort_type {
            99 => cursor = ((page_no - 1) * page_size).to_string(),
            2 => cursor = format!("normalHot#{}", (page_no - 1) * page_size),
            3 => cursor = query.cursor.clone().unwrap_or_else(|| "0".to_string()),
            _ => {}
        }

        let data = json!({
            "threadId": thread_id,
            "pageNo": page_no,
            "showInner": query.show_inner.unwrap_or(true),
            "pageSize": page_size,
            "cursor": cursor,
            "sortType": sort_type,
        });

        create_request(
            "/api/v2/resource/comments",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(comment_new, CommentNew);


// const { resourceTypeMap } = require('../util/config.json')
// // 评论
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.type = resourceTypeMap[query.type]
//   const threadId = query.type + query.id
//   const pageSize = query.pageSize || 20
//   const pageNo = query.pageNo || 1
//   let sortType = Number(query.sortType) || 99
//   if (sortType === 1) {
//     sortType = 99
//   }
//   let cursor = ''
//   switch (sortType) {
//     case 99:
//       cursor = (pageNo - 1) * pageSize
//       break
//     case 2:
//       cursor = 'normalHot#' + (pageNo - 1) * pageSize
//       break
//     case 3:
//       cursor = query.cursor || '0'
//       break
//     default:
//       break
//   }
//   const data = {
//     threadId: threadId,
//     pageNo,
//     showInner: query.showInner || true,
//     pageSize,
//     cursor: cursor,
//     sortType: sortType, //99:按推荐排序,2:按热度排序,3:按时间排序
//   }
//   return request(`/api/v2/resource/comments`, data, createOption(query))
// }
