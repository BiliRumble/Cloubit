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

// 相关视频
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/related/allvideo").route(web::get().to(related_allvideo)));
}

// 入参
define_request_struct!(RelatedAllVideo, {
    id: String,
});

impl RelatedAllVideo {
    async fn requests(req: HttpRequest, query: Query<RelatedAllVideo>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "type": if query.id.parse::<u64>().is_ok() { 0 } else { 1 },
        });
        create_request(
            "/api/cloudvideo/v1/allvideo/rcmd",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}

cache_handler!(related_allvideo, RelatedAllVideo);

// // 相关视频
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     type: /^\d+$/.test(query.id) ? 0 : 1,
//   }
//   return request(
//     `/api/cloudvideo/v1/allvideo/rcmd`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
