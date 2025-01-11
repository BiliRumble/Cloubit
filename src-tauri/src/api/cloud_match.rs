use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;


// 云盘歌曲匹配
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/cloud/match").route(web::post().to(cloud_match)));
}

// 入参
define_request_struct!(CloudMatch, {
    uid: String,
    sid: String,
    asid: String,
});

impl CloudMatch {
    async fn requests(req: HttpRequest, query: Query<CloudMatch>) -> Result<Response, Value> {
        let data = json!({
            "userId": query.uid,
            "songId": query.sid,
            "adjustSongId": query.asid,
        });
        create_request(
            "/api/cloud/user/song/match",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(cloud_match, CloudMatch);


// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     userId: query.uid,
//     songId: query.sid,
//     adjustSongId: query.asid,
//   }
//   return request(
//     `/api/cloud/user/song/match`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }