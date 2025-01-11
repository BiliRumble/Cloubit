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

// // 一起听 房间情况
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/listentogether/room/check").route(web::get().to(listentogether_room_check)));
}

// 入参
define_request_struct!(ListentogetherRomeCheck, {
    roomId: String,
});

impl ListentogetherRomeCheck {
  async fn requests(req: HttpRequest, query: Query<ListentogetherRomeCheck>) -> Result<Response, Value> {
    let data = json!({
        "roomId": query.roomId,
    });
    create_request(
      "/api/listen/together/room/check",
      data,
      create_request_option(extract_headers!(req), &query.common, ""),
    ).await
  }
}
cache_handler!(listentogether_room_check, ListentogetherRomeCheck);


// // 一起听 房间情况
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     roomId: query.roomId,
//   }
//   return request(`/api/listen/together/room/check`, data, createOption(query))
// }
