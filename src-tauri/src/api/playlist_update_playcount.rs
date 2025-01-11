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

// 歌单打卡
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/update/playcount").route(web::get().to(playlist_update_playcount)));
}

// 入参
define_request_struct!(PlaylistUpdatePlaycount, {
    id: String,
});

impl PlaylistUpdatePlaycount {
    async fn requests(req: HttpRequest, query: Query<PlaylistUpdatePlaycount>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
        });
        create_request(
            "/api/playlist/update/playcount",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(playlist_update_playcount, PlaylistUpdatePlaycount);


// // 歌单打卡
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//   }
//   return request(
//     `/api/playlist/update/playcount`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }