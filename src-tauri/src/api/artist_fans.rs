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

// 歌手粉丝
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/artist/fans").route(web::get().to(artist_fans)));
}

// 入参
define_request_struct!(ArtistFans, {
    id: String,
    offset: Option<String>,
    limit: Option<String>,
});


impl ArtistFans {
    async fn requests(req: HttpRequest, query: Query<ArtistFans>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "limit": query.limit.clone().unwrap_or("20".to_string()),
            "offset": query.offset.clone().unwrap_or("0".to_string()),
        });
        create_request(
            "/api/artist/fans/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(artist_fans, ArtistFans);


// 歌手粉丝

// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     limit: query.limit || 20,
//     offset: query.offset || 0,
//   }
//   return request(`/api/artist/fans/get`, data, createOption(query, 'weapi'))
// }
