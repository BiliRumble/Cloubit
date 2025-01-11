
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

// // 最近播放专辑
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/record/recent/album").route(web::get().to(record_recent_album)));
}

// 入参
define_request_struct!(RecordRecentAlbum, {
    limit: Option<u32>,
});

impl RecordRecentAlbum {
    async fn requests(req: HttpRequest, query: Query<RecordRecentAlbum>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.unwrap_or(100),
        });
        create_request(
            "/api/play-record/album/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(record_recent_album, RecordRecentAlbum);

// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 100,
//   }
//   return request(
//     `/api/play-record/album/list`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
