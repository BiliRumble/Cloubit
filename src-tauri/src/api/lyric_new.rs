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


// 新版歌词 - 包含逐字歌词
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/lyric/new").route(web::get().to(lyric_new)));
}

// 入参
define_request_struct!(LyricNew, {
    id: String,
});

impl LyricNew {
    async fn requests(req: HttpRequest, query: Query<LyricNew>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "cp": false,
            "tv": 0,
            "lv": 0,
            "rv": 0,
            "kv": 0,
            "yv": 0,
            "ytv": 0,
            "yrv": 0,
        });
        create_request(
            "/api/song/lyric/v1",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(lyric_new, LyricNew);



// // 新版歌词 - 包含逐字歌词
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     cp: false,
//     tv: 0,
//     lv: 0,
//     rv: 0,
//     kv: 0,
//     yv: 0,
//     ytv: 0,
//     yrv: 0,
//   }
//   return request(`/api/song/lyric/v1`, data, createOption(query))
// }