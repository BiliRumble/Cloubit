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

// // 一起听 结束房间
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/listentogether/end").route(web::get().to(listentogether_end)));
}

// 入参
define_request_struct!(ListentogetherEnd, {
    room_id: String,
});

impl ListentogetherEnd {
    async fn requests(req: HttpRequest, query: Query<ListentogetherEnd>) -> Result<Response, Value> {
        let data = json!({
        "roomId": query.room_id,
    });
        create_request(
            "/api/listen/together/end/v2",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(listentogether_end, ListentogetherEnd);

// // 一起听 结束房间
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     roomId: query.roomId,
//   }
//   return request(`/api/listen/together/end/v2`, data, createOption(query))
// }
