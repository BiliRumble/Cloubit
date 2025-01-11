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

// 获取客户端歌曲下载链接 - v1
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/song/download/url/v1").route(web::post().to(song_download_url_v1)));
}

// 入参
define_request_struct!(SongDownloadUrlV1, {
    id: String,
    level: String,
});

impl SongDownloadUrlV1 {
    async fn requests(req: HttpRequest, query: Query<SongDownloadUrlV1>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "immerseType": "c51",
            "level": query.level,
        });
        create_request(
            "/api/song/enhance/download/url/v1",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(song_download_url_v1, SongDownloadUrlV1);



// // 获取客户端歌曲下载链接 - v1
// // 此版本不再采用 br 作为音质区分的标准
// // 而是采用 standard, exhigh, lossless, hires, jyeffect(高清环绕声), sky(沉浸环绕声), jymaster(超清母带) 进行音质判断
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     immerseType: 'c51',
//     level: query.level,
//   }
//   return request(`/api/song/enhance/download/url/v1`, data, createOption(query))
// }