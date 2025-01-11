
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

// 推荐视频
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/video/timeline/recommend").route(web::get().to(video_timeline_recommend)));
}

// 入参
define_request_struct!(VideoTimelineRecommend, {
  offset: Option<u32>
});

impl VideoTimelineRecommend {
    async fn requests(req: HttpRequest, query: Query<VideoTimelineRecommend>) -> Result<Response, Value> {
        let data = json!({
            "offset": query.offset.unwrap_or(0),
            "filterLives": "[]",
            "withProgramInfo": "true",
            "needUrl": "1",
            "resolution": "480",
        });
        create_request(
            "/api/videotimeline/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(video_timeline_recommend, VideoTimelineRecommend);



// // 推荐视频
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     offset: query.offset || 0,
//     filterLives: '[]',
//     withProgramInfo: 'true',
//     needUrl: '1',
//     resolution: '480',
//   }
//   return request(`/api/videotimeline/get`, data, createOption(query, 'weapi'))
// }