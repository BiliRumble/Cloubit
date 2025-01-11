
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

// 歌曲链接 - v1
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/song/url/v1").route(web::get().to(song_url_v1)));
}

// 入参
define_request_struct!(SongUrlV1, {
    id: String,
    level: String,
});

impl SongUrlV1 {
    async fn requests(req: HttpRequest, query: Query<SongUrlV1>) -> Result<Response, Value> {
        let mut data = json!({
            "ids": format!("[{}]", query.id),
            "level": query.level,
            "encodeType": "flac",
        });
        if query.level == "sky" {
            data["immerseType"] = json!("c51");
        }
        create_request(
            "/api/song/enhance/player/url/v1",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(song_url_v1, SongUrlV1);


// // 歌曲链接 - v1
// // 此版本不再采用 br 作为音质区分的标准
// // 而是采用 standard, exhigh, lossless, hires, jyeffect(高清环绕声), sky(沉浸环绕声), jymaster(超清母带) 进行音质判断
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     ids: '[' + query.id + ']',
//     level: query.level,
//     encodeType: 'flac',
//   }
//   if (data.level == 'sky') {
//     data.immerseType = 'c51'
//   }
//   return request(`/api/song/enhance/player/url/v1`, data, createOption(query))
// }