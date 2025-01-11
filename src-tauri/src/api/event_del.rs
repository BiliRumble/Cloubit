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

// // 删除动态
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/event/del").route(web::get().to(event_del)));
}

// 入参
define_request_struct!(EventDel, {
    evId: String,
});

impl EventDel {
    async fn requests(req: HttpRequest, query: Query<EventDel>) -> Result<Response, Value> {
        let data = json!({
        "id": query.evId,
    });
        create_request(
            "/api/event/delete",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(event_del, EventDel);


// // 删除动态
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.evId,
//   }
//   return request(`/api/event/delete`, data, createOption(query, 'weapi'))
// }
