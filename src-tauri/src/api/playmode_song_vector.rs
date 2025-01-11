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

// 云随机播放
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playmode/song/vector").route(web::get().to(playmode_song_vector)));
}

// 入参
define_request_struct!(PlaymodeSongVector, {
    ids: Vec<String>,
});

impl PlaymodeSongVector {
    async fn requests(req: HttpRequest, query: Query<PlaymodeSongVector>) -> Result<Response, Value> {
        let data = json!({
            "ids": query.ids,
        });
        create_request(
            "/api/playmode/song/vector/get",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(playmode_song_vector, PlaymodeSongVector);



// // 云随机播放
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     ids: query.ids,
//   }
//   return request(`/api/playmode/song/vector/get`, data, createOption(query))
// }