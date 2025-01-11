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
// 回忆坐标
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/music/first/listen/info").route(web::get().to(music_first_listen_info)));
}

// 入参
define_request_struct!(MusicFirstListenInfo, {
    id: String,
});

impl MusicFirstListenInfo {
    async fn requests(req: HttpRequest, query: Query<MusicFirstListenInfo>) -> Result<Response, Value> {
        let data = json!({
            "songId": query.id,
        });
        create_request(
            "/api/content/activity/music/first/listen/info",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(music_first_listen_info, MusicFirstListenInfo);


// // 回忆坐标
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     songId: query.id,
//   }
//   return request(
//     `/api/content/activity/music/first/listen/info`,
//     data,
//     createOption(query),
//   )
// }