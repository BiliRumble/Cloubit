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

// 曲风-歌曲
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/style/song").route(web::get().to(style_song)));
}

// 入参
define_request_struct!(StyleSong, {
    cursor: Option<u32>,
    size: Option<u32>,
    tagId: String,
    sort: Option<u32>,
});

impl StyleSong {
    async fn requests(req: HttpRequest, query: Query<StyleSong>) -> Result<Response, Value> {
        let data = json!({
            "cursor": query.cursor.unwrap_or(0),
            "size": query.size.unwrap_or(20),
            "tagId": query.tagId,
            "sort": query.sort.unwrap_or(0),
        });
        create_request(
            "/api/style-tag/home/song",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(style_song, StyleSong);


// // 曲风-歌曲
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     cursor: query.cursor || 0,
//     size: query.size || 20,
//     tagId: query.tagId,
//     sort: query.sort || 0,
//   }
//   return request(`/api/style-tag/home/song`, data, createOption(query, 'weapi'))
// }