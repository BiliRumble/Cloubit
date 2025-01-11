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


// 精品歌单
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/top/playlist/highquality").route(web::get().to(top_playlist_highquality)));
}

// 入参
define_request_struct!(TopPlaylistHighquality, {
    cat: Option<String>,
    limit: Option<u8>,
    before: Option<u64>,
});

impl TopPlaylistHighquality {
    async fn requests(req: HttpRequest, query: Query<TopPlaylistHighquality>) -> Result<Response, Value> {
        let data = json!({
            "cat": query.cat.clone().unwrap_or("全部".to_string()),
            "limit": query.limit.unwrap_or(50),
            "lasttime": query.before.unwrap_or(0),
            "total": true,
        });
        create_request(
            "/api/playlist/highquality/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(top_playlist_highquality, TopPlaylistHighquality);


// // 精品歌单
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     cat: query.cat || '全部', // 全部,华语,欧美,韩语,日语,粤语,小语种,运动,ACG,影视原声,流行,摇滚,后摇,古风,民谣,轻音乐,电子,器乐,说唱,古典,爵士
//     limit: query.limit || 50,
//     lasttime: query.before || 0, // 歌单updateTime
//     total: true,
//   }
//   return request(
//     `/api/playlist/highquality/list`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }