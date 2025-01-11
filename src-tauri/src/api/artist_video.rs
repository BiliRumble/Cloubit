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

// // 歌手热门 50 首歌曲
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/artist/video").route(web::get().to(artist_video)));
}

// 入参
define_request_struct!(ArtistVideo, {
    id: String,
    size: Option<i32>,
    cursor: Option<i32>,
    order: Option<i32>,
});


impl ArtistVideo {
  async fn requests(req: HttpRequest, query: Query<ArtistVideo>) -> Result<Response, Value> {
    // 创建请求数据
    let data = json!({
        "artistId": query.id,
        "page": {
            "size": query.size.unwrap_or(10),
            "cursor": query.cursor.unwrap_or(0),
        },
        "tab": 0,
        "order": query.order.unwrap_or(0),
    });

    create_request(
      "/api/mlog/artist/video",
      data,
      create_request_option(extract_headers!(req), &query.common, "weapi"),
    ).await
  }
}
cache_handler!(artist_video, ArtistVideo);



// // 歌手相关视频
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     artistId: query.id,
//     page: JSON.stringify({
//       size: query.size || 10,
//       cursor: query.cursor || 0,
//     }),
//     tab: 0,
//     order: query.order || 0,
//   }
//   return request(`/api/mlog/artist/video`, data, createOption(query, 'weapi'))
// }
