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


// // 歌单收藏者
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/subscribers").route(web::post().to(playlist_subscribers)));
}


define_request_struct!(PlaylistSubscribers, {
    id: String,
    limit: Option<u32>,
    offset: Option<u32>,
});

impl PlaylistSubscribers {
    async fn requests(req: HttpRequest, query: Query<PlaylistSubscribers>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "limit": query.limit.unwrap_or(20),
            "offset": query.offset.unwrap_or(0),
        });

        create_request(
            "/api/playlist/subscribers",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}

cache_handler!(playlist_subscribers, PlaylistSubscribers);


// // 歌单收藏者
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     limit: query.limit || 20,
//     offset: query.offset || 0,
//   }
//   return request(
//     `/api/playlist/subscribers`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }