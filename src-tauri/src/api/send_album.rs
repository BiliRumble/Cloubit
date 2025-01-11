
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

// 私信专辑
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/send/album").route(web::post().to(send_album)));
}

// 入参
define_request_struct!(SendAlbum, {
    id: String,
    msg: Option<String>,
    user_ids: String,
});

impl SendAlbum {
    async fn requests(req: HttpRequest, query: Query<SendAlbum>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "msg": query.msg.clone().unwrap_or_default(),
            "type": "album",
            "userIds": format!("[{}]", query.user_ids),
        });
        create_request(
            "/api/msg/private/send",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(send_album, SendAlbum);



// // 私信专辑
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     msg: query.msg || '',
//     type: 'album',
//     userIds: '[' + query.user_ids + ']',
//   }
//   return request(`/api/msg/private/send`, data, createOption(query))
// }