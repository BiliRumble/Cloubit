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

// 歌曲动态封面
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/song/dynamic/cover").route(web::get().to(song_dynamic_cover)));
}

// 入参
define_request_struct!(SongDynamicCover, {
    id: String,
});

impl SongDynamicCover {
    async fn requests(req: HttpRequest, query: Query<SongDynamicCover>) -> Result<Response, Value> {
        let data = json!({
            "songId": query.id,
        });
        create_request(
            "/api/songplay/dynamic-cover",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(song_dynamic_cover, SongDynamicCover);


// // 歌曲动态封面
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     songId: query.id,
//   }
//   return request(`/api/songplay/dynamic-cover`, data, createOption(query))
// }