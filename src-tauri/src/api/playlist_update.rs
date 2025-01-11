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

// 编辑歌单
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/update").route(web::get().to(playlist_update)));
}

// 入参
define_request_struct!(PlaylistUpdate, {
    id: String,
    desc: Option<String>,
    tags: Option<String>,
    name: String,
});

impl PlaylistUpdate {
    async fn requests(req: HttpRequest, query: Query<PlaylistUpdate>) -> Result<Response, Value> {
        let data = json!({
            "/api/playlist/desc/update": json!({"id": query.id, "desc": query.desc.clone().unwrap_or_default()}),
            "/api/playlist/tags/update": json!({"id": query.id, "tags": query.tags.clone().unwrap_or_default()}),
            "/api/playlist/update/name": json!({"id": query.id, "name": query.name}),
        });
        create_request(
            "/api/batch",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(playlist_update, PlaylistUpdate);


// // 编辑歌单
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.desc = query.desc || ''
//   query.tags = query.tags || ''
//   const data = {
//     '/api/playlist/desc/update': `{"id":${query.id},"desc":"${query.desc}"}`,
//     '/api/playlist/tags/update': `{"id":${query.id},"tags":"${query.tags}"}`,
//     '/api/playlist/update/name': `{"id":${query.id},"name":"${query.name}"}`,
//   }
//   return request(`/api/batch`, data, createOption(query, 'weapi'))
// }