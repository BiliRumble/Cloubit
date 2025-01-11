
use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

// 视频标签列表
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/video/group/list").route(web::get().to(video_group_list)));
}

// 入参
define_request_struct!(VideoGroupList, {});

impl VideoGroupList {
    async fn requests(req: HttpRequest, query: Query<VideoGroupList>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/cloudvideo/group/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(video_group_list, VideoGroupList);


// // 视频标签列表
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(
//     `/api/cloudvideo/group/list`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }