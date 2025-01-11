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

// 相似歌曲
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/simi/song").route(web::get().to(simi_song)));
}

// 入参
define_request_struct!(SimiSong, {
    id: String,
    limit: Option<u32>,
    offset: Option<u32>,
});

impl SimiSong {
    async fn requests(req: HttpRequest, query: Query<SimiSong>) -> Result<Response, Value> {
        let data = json!({
            "songid": query.id,
            "limit": query.limit.unwrap_or(50),
            "offset": query.offset.unwrap_or(0),
        });
        create_request(
            "/api/v1/discovery/simiSong",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}

cache_handler!(simi_song, SimiSong);

// // 相似歌曲
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     songid: query.id,
//     limit: query.limit || 50,
//     offset: query.offset || 0,
//   }
//   return request(
//     `/api/v1/discovery/simiSong`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
