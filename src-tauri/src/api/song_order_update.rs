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

// 更新歌曲顺序
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/song/order/update").route(web::get().to(song_order_update)));
}

// 入参
define_request_struct!(SongOrderUpdate, {
    pid: String,
    ids: Vec<String>,
});

impl SongOrderUpdate {
    async fn requests(req: HttpRequest, query: Query<SongOrderUpdate>) -> Result<Response, Value> {
        let data = json!({
            "pid": query.pid,
            "trackIds": query.ids,
            "op": "update",
        });
        create_request(
            "/api/playlist/manipulate/tracks",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(song_order_update, SongOrderUpdate);


// // 更新歌曲顺序
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     pid: query.pid,
//     trackIds: query.ids,
//     op: 'update',
//   }
// 
//   return request(`/api/playlist/manipulate/tracks`, data, createOption(query))
// }