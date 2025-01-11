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

// // 一起听 发送播放状态
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/listentogether/play/command").route(web::get().to(listentogether_play_command)));
}

// 入参
define_request_struct!(ListentogetherPlayCommand, {
    roomId: i64,
    commandType: i64,
    progress: Option<i64>,
    playStatus: i64,
    formerSongId: i64,
    targetSongId: i64,
    clientSeq: i64,
    
});

impl ListentogetherPlayCommand {
    async fn requests(req: HttpRequest, query: Query<ListentogetherPlayCommand>) -> Result<Response, Value> {
        let data = json!({
        "roomId": query.roomId,
        "commandInfo": {
            "commandType": query.commandType,
            "progress": query.progress.unwrap_or(0),
            "playStatus": query.playStatus,
            "formerSongId": query.formerSongId,
            "targetSongId": query.targetSongId,
            "clientSeq": query.clientSeq,
        }
    });
        create_request(
            "/api/listen/together/play/command/report",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(listentogether_play_command, ListentogetherPlayCommand);


// // 一起听 发送播放状态
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     roomId: query.roomId,
//     commandInfo: JSON.stringify({
//       commandType: query.commandType,
//       progress: query.progress || 0,
//       playStatus: query.playStatus,
//       formerSongId: query.formerSongId,
//       targetSongId: query.targetSongId,
//       clientSeq: query.clientSeq,
//     }),
//   }
//   return request(
//     `/api/listen/together/play/command/report`,
//     data,
//     createOption(query),
//   )
// }
