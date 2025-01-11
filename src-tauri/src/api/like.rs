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

// // 红心与取消红心歌曲
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/like").route(web::get().to(like)));
}

// 入参
define_request_struct!(Like, {
    id: String,
    like: String,
});

impl Like {
    async fn requests(req: HttpRequest, query: Query<Like>) -> Result<Response, Value> {
        let like = if query.like == "false" { false } else { true };
        let data = json!({
            "alg": "itembased",
            "trackId": query.id,
            "like": like,
            "time": "3",
        });
        create_request(
            "/api/radio/like",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(like, Like);


// // 红心与取消红心歌曲
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.like = query.like == 'false' ? false : true
//   const data = {
//     alg: 'itembased',
//     trackId: query.id,
//     like: query.like,
//     time: '3',
//   }
//   return request(`/api/radio/like`, data, createOption(query, 'weapi'))
// }
