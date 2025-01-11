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


// 收藏与取消收藏视频
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/video/sub").route(web::get().to(video_sub)));
}

// 入参
define_request_struct!(VideoSub, {
  id: String,
  t: u8
});

impl VideoSub {
    async fn requests(req: HttpRequest, query: Query<VideoSub>) -> Result<Response, Value> {
        let action = if query.t == 1 { "sub" } else { "unsub" };
        let data = json!({
            "id": query.id,
        });
        create_request(
            &format!("/api/cloudvideo/video/{}", action),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(video_sub, VideoSub);



// // 收藏与取消收藏视频
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.t = query.t == 1 ? 'sub' : 'unsub'
//   const data = {
//     id: query.id,
//   }
//   return request(
//     `/api/cloudvideo/video/${query.t}`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }