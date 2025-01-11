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


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/mylike").route(web::get().to(playlist_mylike)));
}

// 入参
define_request_struct!(PlaylistMyLike, {
    time: Option<String>,
    limit: Option<String>,
});

impl PlaylistMyLike {
    async fn requests(req: HttpRequest, query: Query<PlaylistMyLike>) -> Result<Response, Value> {
        let data = json!({
            "time": query.time.clone().unwrap_or_else(|| "-1".to_string()),
            "limit": query.limit.clone().unwrap_or_else(|| "12".to_string()),
        });
        create_request(
            "/api/mlog/playlist/mylike/bytime/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}


cache_handler!(playlist_mylike, PlaylistMyLike);

// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     time: query.time || '-1',
//     limit: query.limit || '12',
//   }
//   return request(
//     `/api/mlog/playlist/mylike/bytime/get`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }