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


// 热门歌手
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/top/artists").route(web::get().to(top_artists)));
}

// 入参
define_request_struct!(TopArtists, {
    limit: Option<u32>,
    offset: Option<u32>,
});

impl TopArtists {
    async fn requests(req: HttpRequest, query: Query<TopArtists>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.unwrap_or(50),
            "offset": query.offset.unwrap_or(0),
            "total": true,
        });
        create_request(
            "/api/artist/top",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(top_artists, TopArtists);



// // 热门歌手
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 50,
//     offset: query.offset || 0,
//     total: true,
//   }
//   return request(`/api/artist/top`, data, createOption(query, 'weapi'))
// }