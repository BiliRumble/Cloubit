
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

// 歌曲简要百科信息
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ugc/song/get").route(web::get().to(ugc_song_get)));
}

// 入参
define_request_struct!(UgcSongGet, {
  songId: String
});


impl UgcSongGet {
    async fn requests(req: HttpRequest, query: Query<UgcSongGet>) -> Result<Response, Value> {
        let data = json!({
          "songId": query.songId
        });
        create_request(
            "/api/rep/ugc/song/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(ugc_song_get, UgcSongGet);


// // 歌曲简要百科信息
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     songId: query.id,
//   }
//   return request(`/api/rep/ugc/song/get`, data, createOption(query))
// }