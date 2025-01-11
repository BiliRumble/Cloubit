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


// 全部视频列表
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/video/timeline/all").route(web::get().to(video_timeline_all)));
}

// 入参
define_request_struct!(VideoTimelineAll, {
  offset: Option<u32>
});

impl VideoTimelineAll {
    async fn requests(req: HttpRequest, query: Query<VideoTimelineAll>) -> Result<Response, Value> {
        let data = json!({
            "groupId": 0,
            "offset": query.offset.unwrap_or(0),
            "need_preview_url": "true",
            "total": true,
        });
        create_request(
            "/api/videotimeline/otherclient/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(video_timeline_all, VideoTimelineAll);



// // 全部视频列表
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     groupId: 0,
//     offset: query.offset || 0,
//     need_preview_url: 'true',
//     total: true,
//   }
//   //   /api/videotimeline/otherclient/get
//   return request(
//     `/api/videotimeline/otherclient/get`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }