
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
    cfg.service(web::resource("/user/playlist").route(web::get().to(user_playlist)));
}

define_request_struct!(UserPlaylist, {
    uid: String,
    limit: Option<i32>,
    offset: Option<i32>,
});

impl UserPlaylist {
    async fn requests(req: HttpRequest, query: Query<UserPlaylist>) -> Result<Response, Value> {
        let data = json!({
            "uid": query.uid,
            "limit": query.limit.unwrap_or(30),
            "offset": query.offset.unwrap_or(0),
            "includeVideo": true,
        });
        create_request(
            "/api/user/playlist",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(user_playlist, UserPlaylist);




// // 用户歌单
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     uid: query.uid,
//     limit: query.limit || 30,
//     offset: query.offset || 0,
//     includeVideo: true,
//   }
//   return request(`/api/user/playlist`, data, createOption(query, 'weapi'))
// }