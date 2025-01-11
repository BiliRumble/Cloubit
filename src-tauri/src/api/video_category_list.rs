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

// 视频分类列表
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/video/category/list").route(web::get().to(video_category_list)));
}

// 入参
define_request_struct!(VideoCategoryList, {
    offset: Option<u32>,
    limit: Option<u32>,
});

impl VideoCategoryList {
    async fn requests(
        req: HttpRequest,
        query: Query<VideoCategoryList>,
    ) -> Result<Response, Value> {
        let data = json!({
            "offset": query.offset.unwrap_or(0),
            "total": "true",
            "limit": query.limit.unwrap_or(99),
        });
        create_request(
            "/api/cloudvideo/category/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}

cache_handler!(video_category_list, VideoCategoryList);

// // 视频分类列表
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     offset: query.offset || 0,
//     total: 'true',
//     limit: query.limit || 99,
//   }
//   return request(
//     `/api/cloudvideo/category/list`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
