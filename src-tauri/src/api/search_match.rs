
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

// 本地歌曲匹配音乐信息
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/search/match").route(web::post().to(search_match)));
}

// 入参
define_request_struct!(SearchMatch, {
    title: Option<String>,
    album: Option<String>,
    artist: Option<String>,
    duration: Option<u32>,
    md5: String,
});

impl SearchMatch {
    async fn requests(req: HttpRequest, query: Query<SearchMatch>) -> Result<Response, Value> {
        let songs = json!([{
            "title": query.title.clone().unwrap_or_default(),
            "album": query.album.clone().unwrap_or_default(),
            "artist": query.artist.clone().unwrap_or_default(),
            "duration": query.duration.unwrap_or(0),
            "persistId": query.md5,
        }]);
        let data = json!({
            "songs": songs.to_string(),
        });
        create_request(
            "/api/search/match/new",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(search_match, SearchMatch);



// // 本地歌曲匹配音乐信息
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   let songs = [
//     {
//       title: query.title || '',
//       album: query.album || '',
//       artist: query.artist || '',
//       duration: query.duration || 0,
//       persistId: query.md5,
//     },
//   ]
//   const data = {
//     songs: JSON.stringify(songs),
//   }
//   return request(`/api/search/match/new`, data, createOption(query))
// }