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

// 音乐百科基础信息
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/song/wiki/summary").route(web::get().to(song_wiki_summary)));
}

// 入参
define_request_struct!(SongWikiSummary, {
    song_id: String,
});

impl SongWikiSummary {
    async fn requests(req: HttpRequest, query: Query<SongWikiSummary>) -> Result<Response, Value> {
        let data = json!({
            "songId": query.song_id,
        });
        create_request(
            "/api/song/play/about/block/page",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(song_wiki_summary, SongWikiSummary);



// // 音乐百科基础信息
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     songId: query.id,
//   }
//   return request(`/api/song/play/about/block/page`, data, createOption(query))
// }
