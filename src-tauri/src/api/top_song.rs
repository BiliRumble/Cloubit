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

// 新歌速递
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/top/song").route(web::get().to(top_song)));
}

// 入参
define_request_struct!(TopSong, {
    r#type: Option<u8>,
});

impl TopSong {
    async fn requests(req: HttpRequest, query: Query<TopSong>) -> Result<Response, Value> {
        let data = json!({
            "areaId": query.r#type.unwrap_or(0),
            "total": true,
        });
        create_request(
            "/api/v1/discovery/new/songs",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(top_song, TopSong);


// // 新歌速递
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     areaId: query.type || 0, // 全部:0 华语:7 欧美:96 日本:8 韩国:16
//     // limit: query.limit || 100,
//     // offset: query.offset || 0,
//     total: true,
//   }
//   return request(
//     `/api/v1/discovery/new/songs`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }