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
use std::time::{SystemTime, UNIX_EPOCH};
use web::Query;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/artist/new/song").route(web::get().to(artist_new_song)));
}

// 入参
define_request_struct!(ArtistNewSong, {
    before: Option<String>,
    limit: Option<String>,
});


impl ArtistNewSong {
    async fn requests(req: HttpRequest, query: Query<ArtistNewSong>) -> Result<Response, Value> {
        // 处理 `before` 参数
        let start_timestamp = if let Some(before) = &query.before {
            before.clone()
        } else {
            // 当前时间戳（毫秒）
            SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis()
                .to_string()
        };

        // 创建请求数据
        let data = json!({
            "limit": query.limit.clone().unwrap_or_else(|| "20".to_string()),
            "startTimestamp": start_timestamp,
        });

        create_request(
            "/api/sub/artist/new/works/song/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(artist_new_song, ArtistNewSong);


// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 20,
//     startTimestamp: query.before || Date.now(),
//   }
//   return request(
//     `/api/sub/artist/new/works/song/list`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }