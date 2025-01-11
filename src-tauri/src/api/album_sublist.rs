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

// // 已收藏专辑列表
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/album/sublist").route(web::get().to(album_sublist)));
}

// 入参
define_request_struct!(AlbumSublist, {
    offset: Option<String>,
    limit: Option<String>,
});


impl AlbumSublist {
    async fn requests(req: HttpRequest, query: Query<AlbumSublist>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.clone().unwrap_or("25".to_string()),
            "offset": query.offset.clone().unwrap_or("0".to_string()),
            "total": true,
        });
        create_request(
            "/api/album/sublist",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(album_sublist, AlbumSublist);


//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 25,
//     offset: query.offset || 0,
//     total: true,
//   }
//   return request(`/api/album/sublist`, data, createOption(query, 'weapi'))
// }