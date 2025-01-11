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

// 歌手专辑列表
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/artist/album").route(web::get().to(artist_album)));
}

// 入参
define_request_struct!(ArtistAlbum, {
    id: String,
    offset: Option<String>,
    limit: Option<String>,
});


impl ArtistAlbum {
    async fn requests(req: HttpRequest, query: Query<ArtistAlbum>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.clone().unwrap_or("30".to_string()),
            "offset": query.offset.clone().unwrap_or("0".to_string()),
            "total": true,
        });
        create_request(
            &format!("/api/artist/albums/{}", query.id),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(artist_album, ArtistAlbum);


// // 歌手专辑列表
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 30,
//     offset: query.offset || 0,
//     total: true,
//   }
//   return request(
//     `/api/artist/albums/${query.id}`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }