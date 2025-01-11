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


// 听歌打卡
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/scrobble").route(web::get().to(scrobble)));
}

// 入参
define_request_struct!(Scrobble, {
    id: String,
    sourceid: String,
    time: String,
});

impl Scrobble {
    async fn requests(req: HttpRequest, query: Query<Scrobble>) -> Result<Response, Value> {
        let data = json!({
            "logs": json!([{
                "action": "play",
                "json": {
                    "download": 0,
                    "end": "playend",
                    "id": query.id,
                    "sourceId": query.sourceid,
                    "time": query.time,
                    "type": "song",
                    "wifi": 0,
                    "source": "list",
                    "mainsite": 1,
                    "content": "",
                },
            }]),
        });
        create_request(
            "/api/feedback/weblog",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}

cache_handler!(scrobble, Scrobble);


// // 听歌打卡
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     logs: JSON.stringify([
//       {
//         action: 'play',
//         json: {
//           download: 0,
//           end: 'playend',
//           id: query.id,
//           sourceId: query.sourceid,
//           time: query.time,
//           type: 'song',
//           wifi: 0,
//           source: 'list',
//           mainsite: 1,
//           content: '',
//         },
//       },
//     ]),
//   }
// 
//   return request(`/api/feedback/weblog`, data, createOption(query, 'weapi'))
// }