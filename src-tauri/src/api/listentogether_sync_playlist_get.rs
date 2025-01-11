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

// // 一起听 当前列表获取
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/listentogether/sync/playlist/get").route(web::get().to(listentogether_sync_playlist_get)));
}

// 入参
define_request_struct!(ListentogetherSyncPlaylistGet, {
    roomId: String,
});

impl ListentogetherSyncPlaylistGet {
  async fn requests(req: HttpRequest, query: Query<ListentogetherSyncPlaylistGet>) -> Result<Response, Value> {
    let data = json!({
        "roomId": query.roomId,
    });
    create_request(
      "/api/listen/together/sync/playlist/get",
      data,
      create_request_option(extract_headers!(req), &query.common, ""),
    ).await
  }
}
cache_handler!(listentogether_sync_playlist_get, ListentogetherSyncPlaylistGet);

// // 一起听 当前列表获取
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     roomId: query.roomId,
//   }
//   return request(
//     `/api/listen/together/sync/playlist/get`,
//     data,
//     createOption(query),
//   )
// }
