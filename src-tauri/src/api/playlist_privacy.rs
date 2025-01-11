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

// // 公开隐私歌单
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/privacy").route(web::post().to(playlist_privacy)));
}


define_request_struct!(PlaylistPrivacy, {
    id: String,
});

impl PlaylistPrivacy {
    async fn requests(req: HttpRequest, query: Query<PlaylistPrivacy>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "privacy": 0,
        });

        create_request(
            "/api/playlist/update/privacy",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}


cache_handler!(playlist_privacy, PlaylistPrivacy);

// // 公开隐私歌单
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     privacy: 0,
//   }
//   return request(`/api/playlist/update/privacy`, data, createOption(query))
// }