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

// 删除歌单
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/delete").route(web::get().to(playlist_delete)));
}

// 入参
define_request_struct!(PlaylistDelete, {
    id: String,
});

impl PlaylistDelete {
    async fn requests(req: HttpRequest, query: Query<PlaylistDelete>) -> Result<Response, Value> {
        let data = json!({
            "ids": format!("[{}]", query.id),
        });
        create_request(
            "/api/playlist/remove",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(playlist_delete, PlaylistDelete);


// // 删除歌单
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     ids: '[' + query.id + ']',
//   }
//   return request(`/api/playlist/remove`, data, createOption(query, 'weapi'))
// }