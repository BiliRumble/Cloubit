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

// // 一起听 发送心跳
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/listentogether/heatbeat").route(web::get().to(listentogether_heatbeat)));
}

// 入参
define_request_struct!(ListentogetherHeatbeat, {
    room_id: String,
    song_id: String,
    play_status: String,
    progress: String,
});

impl ListentogetherHeatbeat {
    async fn requests(req: HttpRequest, query: Query<ListentogetherHeatbeat>) -> Result<Response, Value> {
        let data = json!({
        "roomId": query.room_id,
        "songId": query.song_id,
        "playStatus": query.play_status,
        "progress": query.progress,
    });
        create_request(
            "/api/listen/together/heartbeat",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(listentogether_heatbeat, ListentogetherHeatbeat);


// // 一起听 发送心跳
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     roomId: query.roomId,
//     songId: query.songId,
//     playStatus: query.playStatus,
//     progress: query.progress,
//   }
//   return request(`/api/listen/together/heartbeat`, data, createOption(query))
// }
