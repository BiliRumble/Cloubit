
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

// 网易出品
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/mv/exclusive/rcmd").route(web::get().to(mv_exclusive_rcmd)));
}

// 入参
define_request_struct!(MvExclusiveRcmd, {
    offset: Option<u32>,
    limit: Option<u32>,
});

impl MvExclusiveRcmd {
    async fn requests(req: HttpRequest, query: Query<MvExclusiveRcmd>) -> Result<Response, Value> {
        let data = json!({
            "offset": query.offset.unwrap_or(0),
            "limit": query.limit.unwrap_or(30),
        });
        create_request(
            "/api/mv/exclusive/rcmd",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(mv_exclusive_rcmd, MvExclusiveRcmd);


// // 网易出品
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     offset: query.offset || 0,
//     limit: query.limit || 30,
//   }
//   return request(`/api/mv/exclusive/rcmd`, data, createOption(query))
// }