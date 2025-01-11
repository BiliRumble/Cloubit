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

// 歌单详情
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/detail").route(web::get().to(playlist_detail)));
}

// 入参
define_request_struct!(PlaylistDetail, {
    id: String,
    s: Option<u32>,
});

impl PlaylistDetail {
    async fn requests(req: HttpRequest, query: Query<PlaylistDetail>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "n": 100000,
            "s": query.s.unwrap_or(8),
        });
        create_request(
            "/api/v6/playlist/detail",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(playlist_detail, PlaylistDetail);

// // 歌单详情
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     n: 100000,
//     s: query.s || 8,
//   }
//   return request(`/api/v6/playlist/detail`, data, createOption(query))
// }