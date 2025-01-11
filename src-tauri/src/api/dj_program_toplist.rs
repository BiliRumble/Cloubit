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

// // 电台节目榜
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/program/toplist").route(web::get().to(dj_program_toplist)));
}

// 入参
define_request_struct!(DjProgramToplist, {
    limit: Option<i32>,
    offset: Option<i32>,
});

impl DjProgramToplist {
    async fn requests(req: HttpRequest, query: Query<DjProgramToplist>) -> Result<Response, Value> {
        let data = json!({
        "limit": query.limit.unwrap_or(100),
        "offset": query.offset.unwrap_or(0),
        });
        create_request(
            "/api/program/toplist/v1",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_program_toplist, DjProgramToplist);


// // 电台节目榜
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 100,
//     offset: query.offset || 0,
//   }
//   return request(`/api/program/toplist/v1`, data, createOption(query, 'weapi'))
// }
