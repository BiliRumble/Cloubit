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

// 创建歌单
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/create").route(web::get().to(playlist_create)));
}

// 入参
define_request_struct!(PlaylistCreate, {
    name: String,
    privacy: u8,
    r#type: Option<String>,
});

impl PlaylistCreate {
    async fn requests(req: HttpRequest, query: Query<PlaylistCreate>) -> Result<Response, Value> {
        let data = json!({
            "name": query.name,
            "privacy": query.privacy,
            "type": query.r#type.clone().unwrap_or_else(|| "NORMAL".to_string()),
        });
        create_request(
            "/api/playlist/create",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(playlist_create, PlaylistCreate);

// // 创建歌单
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     name: query.name,
//     privacy: query.privacy, //0 为普通歌单，10 为隐私歌单
//     type: query.type || 'NORMAL', // NORMAL|VIDEO|SHARED
//   }
//   return request(`/api/playlist/create`, data, createOption(query, 'weapi'))
// }