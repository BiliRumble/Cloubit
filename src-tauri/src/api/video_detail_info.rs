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

// 视频点赞转发评论数数据
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/video/detail/info").route(web::get().to(video_detail_info)));
}

// 入参
define_request_struct!(VideoDetailInfo, {
    vid: String,
});

impl VideoDetailInfo {
    async fn requests(req: HttpRequest, query: Query<VideoDetailInfo>) -> Result<Response, Value> {
        let data = json!({
            "threadid": format!("R_VI_62_{}", query.vid),
            "composeliked": true,
        });
        create_request(
            "/api/comment/commentthread/info",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}


cache_handler!(video_detail_info, VideoDetailInfo);


// // 视频点赞转发评论数数据
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     threadid: `R_VI_62_${query.vid}`,
//     composeliked: true,
//   }
//   return request(
//     `/api/comment/commentthread/info`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }