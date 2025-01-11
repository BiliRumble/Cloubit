
use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

// 获取歌词
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/voice/lyric").route(web::get().to(voice_lyric)));
}

// 入参
define_request_struct!(VoiceLyric, {
    id: String
});

impl VoiceLyric {
    async fn requests(req: HttpRequest, query: Query<VoiceLyric>) -> Result<Response, Value> {
        let data = json!({
            "programId": query.id
        });
        create_request(
            "/api/voice/lyric/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(voice_lyric, VoiceLyric);


// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     programId: query.id,
//   }
//   return request(`/api/voice/lyric/get`, data, createOption(query))
// }