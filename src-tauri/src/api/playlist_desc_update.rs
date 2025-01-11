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

// 更新歌单描述
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/desc/update").route(web::get().to(playlist_desc_update)));
}

// 入参
define_request_struct!(PlaylistDescUpdate, {
    id: String,
    desc: String,
});

impl PlaylistDescUpdate {
    async fn requests(req: HttpRequest, query: Query<PlaylistDescUpdate>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "desc": query.desc,
        });
        create_request(
            "/api/playlist/desc/update",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(playlist_desc_update, PlaylistDescUpdate);

// // 更新歌单描述
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     desc: query.desc,
//   }
//   return request(`/api/playlist/desc/update`, data, createOption(query))
// }