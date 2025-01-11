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

// 智能播放
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playmode/intelligence/list").route(web::get().to(playmode_intelligence_list)));
}

// 入参
define_request_struct!(PlaymodeIntelligenceList, {
    songId: String,
    playlistId: String,
    startMusicId: Option<String>,
    count: Option<u32>,
});

impl PlaymodeIntelligenceList {
    async fn requests(req: HttpRequest, query: Query<PlaymodeIntelligenceList>) -> Result<Response, Value> {
        let data = json!({
            "songId": query.songId,
            "type": "fromPlayOne",
            "playlistId": query.playlistId,
            "startMusicId": query.startMusicId.clone().unwrap_or_else(|| query.songId.clone()),
            "count": query.count.unwrap_or(1),
        });
        create_request(
            "/api/playmode/intelligence/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(playmode_intelligence_list, PlaymodeIntelligenceList);




// // 智能播放
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     songId: query.id,
//     type: 'fromPlayOne',
//     playlistId: query.pid,
//     startMusicId: query.sid || query.id,
//     count: query.count || 1,
//   }
//   return request(
//     `/api/playmode/intelligence/list`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }