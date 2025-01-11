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

// 相似歌手
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/simi/artist").route(web::get().to(simi_artist)));
}

// 入参
define_request_struct!(SimiArtist, {
    id: String,
});

impl SimiArtist {
    async fn requests(req: HttpRequest, query: Query<SimiArtist>) -> Result<Response, Value> {
        let data = json!({
            "artistid": query.id,
        });
        create_request(
            "/api/discovery/simiArtist",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}

cache_handler!(simi_artist, SimiArtist);

// // 相似歌手
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     artistid: query.id,
//   }
//   return request(
//     `/api/discovery/simiArtist`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
