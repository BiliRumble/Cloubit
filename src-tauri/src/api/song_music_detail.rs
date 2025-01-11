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

// 歌曲音质详情
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/song/music/detail").route(web::get().to(song_music_detail)));
}

// 入参
define_request_struct!(SongMusicDetail, {
    id: String,
});

impl SongMusicDetail {
    async fn requests(req: HttpRequest, query: Query<SongMusicDetail>) -> Result<Response, Value> {
        let data = json!({
            "songId": query.id,
        });
        create_request(
            "/api/song/music/detail/get",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(song_music_detail, SongMusicDetail);


// // 歌曲音质详情
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     songId: query.id,
//   }
//   return request(`/api/song/music/detail/get`, data, createOption(query))
// }