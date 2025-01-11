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
use std::time::{SystemTime, UNIX_EPOCH};
use web::Query;

// // 歌手相关MV
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/artist/songs").route(web::get().to(artist_songs)));
}

// 入参
define_request_struct!(ArtistSongs, {
    id: String,
    order: Option<String>,
    offset: Option<i32>,
    limit: Option<i32>,
});


impl ArtistSongs {
  async fn requests(req: HttpRequest, query: Query<ArtistSongs>) -> Result<Response, Value> {
    
    // 创建请求数据
    let data = json!({
            "id": query.id,
            "private_cloud": "true",
            "work_type": 1,
            "order": query.order.clone().unwrap_or("hot".to_string()), //hot,time
            "offset": query.offset.unwrap_or(0),
            "limit": query.limit.unwrap_or(100),
        });

    create_request(
      "/api/v1/artist/songs",
      data,
      create_request_option(extract_headers!(req), &query.common, ""),
    ).await
  }
}
cache_handler!(artist_songs, ArtistSongs);




// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     private_cloud: 'true',
//     work_type: 1,
//     order: query.order || 'hot', //hot,time
//     offset: query.offset || 0,
//     limit: query.limit || 100,
//   }
//   return request(`/api/v1/artist/songs`, data, createOption(query))
// }
