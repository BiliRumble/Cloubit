
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



// 视频详情
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/video/group").route(web::get().to(video_group)));
}


define_request_struct!(VideoGroup, {
    id: String,
    offset: Option<u32>,
});

impl VideoGroup {
    async fn requests(req: HttpRequest, query: Query<VideoGroup>) -> Result<Response, Value> {
        let data = json!({
            "groupId": query.id,
            "offset": query.offset.unwrap_or(0),
            "need_preview_url": "true",
            "total": true,
        });
        create_request(
            "/api/videotimeline/videogroup/otherclient/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}

cache_handler!(video_group, VideoGroup);


// // 视频标签/分类下的视频
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     groupId: query.id,
//     offset: query.offset || 0,
//     need_preview_url: 'true',
//     total: true,
//   }
//   return request(
//     `/api/videotimeline/videogroup/otherclient/get`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }