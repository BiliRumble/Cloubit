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

// 获取客户端歌曲下载链接
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/song/download/url").route(web::post().to(song_download_url)));
}

// 入参
define_request_struct!(SongDownloadUrl, {
    id: String,
    br: Option<i32>,
});

impl SongDownloadUrl {
    async fn requests(req: HttpRequest, query: Query<SongDownloadUrl>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "br": query.br.unwrap_or(999000),
        });
        create_request(
            "/api/song/enhance/download/url",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(song_download_url, SongDownloadUrl);



// // 获取客户端歌曲下载链接
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     br: parseInt(query.br || 999000),
//   }
//   return request(`/api/song/enhance/download/url`, data, createOption(query))
// }