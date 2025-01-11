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


// 编辑歌单顺序
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/order/update").route(web::post().to(playlist_order_update)));
}

// 入参
define_request_struct!(PlaylistOrderUpdate, {
    ids: String,
});

impl PlaylistOrderUpdate {
    async fn requests(req: HttpRequest, query: Query<PlaylistOrderUpdate>) -> Result<Response, Value> {
        let data = json!({
            "ids": query.ids,
        });
        create_request(
            "/api/playlist/order/update",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}




cache_handler!(playlist_order_update, PlaylistOrderUpdate);

// // 编辑歌单顺序
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     ids: query.ids,
//   }
//   return request(
//     `/api/playlist/order/update`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }