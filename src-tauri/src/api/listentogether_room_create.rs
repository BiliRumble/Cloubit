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

// // 一起听创建房间
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/listentogether/room/create").route(web::get().to(listentogether_room_create)));
}

// 入参
define_request_struct!(ListentogetherRomeCreate, {

});

impl ListentogetherRomeCreate {
    async fn requests(req: HttpRequest, query: Query<ListentogetherRomeCreate>) -> Result<Response, Value> {
        let data = json!({
        "refer": "songplay_more",
    });
        create_request(
            "/api/listen/together/room/create",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(listentogether_room_create, ListentogetherRomeCreate);


// // 一起听创建房间
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     refer: 'songplay_more',
//   }
//   return request(`/api/listen/together/room/create`, data, createOption(query))
// }
// //