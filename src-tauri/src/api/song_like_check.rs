
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

// 歌曲是否喜爱
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/song/like/check").route(web::get().to(song_like_check)));
}

// 入参
define_request_struct!(SongLikeCheck, {
    ids: Vec<String>,
});

impl SongLikeCheck {
    async fn requests(req: HttpRequest, query: Query<SongLikeCheck>) -> Result<Response, Value> {
        let data = json!({
            "trackIds": query.ids,
        });
        create_request(
            "/api/song/like/check",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(song_like_check, SongLikeCheck);



// // 歌曲是否喜爱
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     trackIds: query.ids,
//   }
//   return request(`/api/song/like/check`, data, createOption(query))
// }