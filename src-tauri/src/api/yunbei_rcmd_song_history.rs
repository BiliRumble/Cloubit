
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

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/yunbei/rcmd/song/history").route(web::get().to(yunbei_rcmd_song_history)));
}

define_request_struct!(YunbeiRcmdSongHistory, {
    size: Option<u32>,
    cursor: Option<String>,
});

impl YunbeiRcmdSongHistory {
    async fn requests(req: HttpRequest, query: Query<YunbeiRcmdSongHistory>) -> Result<Response, Value> {
        let data = json!({
            "page": json!({
                "size": query.size.unwrap_or(20),
                "cursor": query.cursor.clone().unwrap_or_else(|| "".to_string()),
            }),
        });
        create_request(
            "/api/yunbei/rcmd/song/history/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(yunbei_rcmd_song_history, YunbeiRcmdSongHistory);


// // 云贝推歌历史记录
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     page: JSON.stringify({
//       size: query.size || 20,
//       cursor: query.cursor || '',
//     }),
//   }
//   return request(
//     `/api/yunbei/rcmd/song/history/list`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }