
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

// 视频链接
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/video/url").route(web::get().to(video_url)));
}

// 入参
define_request_struct!(VideoUrl, {
  id: String,
  res: Option<u32>
});

impl VideoUrl {
    async fn requests(req: HttpRequest, query: Query<VideoUrl>) -> Result<Response, Value> {
        let data = json!({
            "ids": format!("[\"{}\"]", query.id),
            "resolution": query.res.unwrap_or(1080),
        });
        create_request(
            "/api/cloudvideo/playurl",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(video_url, VideoUrl);



// // 视频链接
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     ids: '["' + query.id + '"]',
//     resolution: query.res || 1080,
//   }
//   return request(`/api/cloudvideo/playurl`, data, createOption(query, 'weapi'))
// }