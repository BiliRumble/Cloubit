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

// 将mlog id转为video id
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/mlog/to/video").route(web::get().to(mlog_to_video)));
}

// 入参
define_request_struct!(MlogToVideo, {
    id: String,
});

impl MlogToVideo {
    async fn requests(req: HttpRequest, query: Query<MlogToVideo>) -> Result<Response, Value> {
        let data = json!({
            "mlogId": query.id,
        });
        create_request(
            "/api/mlog/video/convert/id",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(mlog_to_video, MlogToVideo);



// // 将mlog id转为video id
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     mlogId: query.id,
//   }
//   return request(
//     `/api/mlog/video/convert/id`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }