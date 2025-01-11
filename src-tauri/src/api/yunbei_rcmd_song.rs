
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
    cfg.service(web::resource("/yunbei/rcmd/song").route(web::get().to(yunbei_rcmd_song)));
}

define_request_struct!(YunbeiRcmdSong, {
    song_id: String,
    reason: Option<String>,
    yunbei_num: Option<u32>,
});

impl YunbeiRcmdSong {
    async fn requests(req: HttpRequest, query: Query<YunbeiRcmdSong>) -> Result<Response, Value> {
        let data = json!({
            "songId": query.song_id,
            "reason": query.reason.clone().unwrap_or_else(|| "好歌献给你".to_string()),
            "scene": "",
            "fromUserId": -1,
            "yunbeiNum": query.yunbei_num.unwrap_or(10),
        });
        create_request(
            "/api/yunbei/rcmd/song/submit",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(yunbei_rcmd_song, YunbeiRcmdSong);


// // 云贝推歌
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     songId: query.id,
//     reason: query.reason || '好歌献给你',
//     scene: '',
//     fromUserId: -1,
//     yunbeiNum: query.yunbeiNum || 10,
//   }
//   return request(
//     `/api/yunbei/rcmd/song/submit`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }