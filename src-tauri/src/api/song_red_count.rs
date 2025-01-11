
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

// 歌曲红心数量
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/song/red/count").route(web::get().to(song_red_count)));
}

// 入参
define_request_struct!(SongRedCount, {
    id: String,
});

impl SongRedCount {
    async fn requests(req: HttpRequest, query: Query<SongRedCount>) -> Result<Response, Value> {
        let data = json!({
            "songId": query.id,
        });
        create_request(
            "/api/song/red/count",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(song_red_count, SongRedCount);



// // 歌曲红心数量
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     songId: query.id,
//   }
//   return request(`/api/song/red/count`, data, createOption(query))
// }