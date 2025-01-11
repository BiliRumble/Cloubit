
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

// 私信歌单
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/send/playlist").route(web::post().to(send_playlist)));
}

// 入参
define_request_struct!(SendPlaylist, {
    playlist: String,
    msg: String,
    user_ids: String,
});

impl SendPlaylist {
    async fn requests(req: HttpRequest, query: Query<SendPlaylist>) -> Result<Response, Value> {
        let data = json!({
            "id": query.playlist,
            "type": "playlist",
            "msg": query.msg,
            "userIds": format!("[{}]", query.user_ids),
        });
        create_request(
            "/api/msg/private/send",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(send_playlist, SendPlaylist);

// // 私信歌单
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.playlist,
//     type: 'playlist',
//     msg: query.msg,
//     userIds: '[' + query.user_ids + ']',
//   }
//   return request(`/api/msg/private/send`, data, createOption(query, 'weapi'))
// }