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

// 歌手榜
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/toplist/artist").route(web::get().to(toplist_artist)));
}

// 入参
define_request_struct!(ToplistArtist, {
    r#type: Option<u8>,
});

impl ToplistArtist {
    async fn requests(req: HttpRequest, query: Query<ToplistArtist>) -> Result<Response, Value> {
        let data = json!({
            "type": query.r#type.unwrap_or(1),
            "limit": 100,
            "offset": 0,
            "total": true,
        });
        create_request(
            "/api/toplist/artist",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(toplist_artist, ToplistArtist);



// // 歌手榜
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     type: query.type || 1,
//     limit: 100,
//     offset: 0,
//     total: true,
//   }
//   return request(`/api/toplist/artist`, data, createOption(query, 'weapi'))
// }