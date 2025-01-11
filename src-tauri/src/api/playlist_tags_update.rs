
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

// // 更新歌单标签
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/tags/update").route(web::post().to(playlist_tags_update)));
}


define_request_struct!(PlaylistTagsUpdate, {
    id: String,
    tags: String,
});

impl PlaylistTagsUpdate {
    async fn requests(req: HttpRequest, query: Query<PlaylistTagsUpdate>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "tags": query.tags,
        });

        create_request(
            "/api/playlist/tags/update",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}

cache_handler!(playlist_tags_update, PlaylistTagsUpdate);


// // 更新歌单标签
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     tags: query.tags,
//   }
//   return request(`/api/playlist/tags/update`, data, createOption(query))
// }