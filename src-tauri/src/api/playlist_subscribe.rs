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


// // 收藏与取消收藏歌单
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/subscribe").route(web::post().to(playlist_subscribe)));
}


define_request_struct!(PlaylistSubscribe, {
    id: String,
    t: i32,
});

impl PlaylistSubscribe {
    async fn requests(req: HttpRequest, query: Query<PlaylistSubscribe>) -> Result<Response, Value> {
        let t = if query.t == 1 { "subscribe" } else { "unsubscribe" };
        let data = json!({
            "id": query.id,
        });

        create_request(
            &format!("/api/playlist/{}", t),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}

cache_handler!(playlist_subscribe, PlaylistSubscribe);


// // 收藏与取消收藏歌单
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.t = query.t == 1 ? 'subscribe' : 'unsubscribe'
//   const data = {
//     id: query.id,
//   }
//   return request(`/api/playlist/${query.t}`, data, createOption(query, 'weapi'))
// }