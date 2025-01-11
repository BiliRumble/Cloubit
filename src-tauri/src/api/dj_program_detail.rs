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

// // 电台节目详情
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/program/detail").route(web::get().to(dj_program_detail)));
}

// 入参
define_request_struct!(DjProgram, {
    id: String,
});

impl DjProgram {
    async fn requests(req: HttpRequest, query: Query<DjProgram>) -> Result<Response, Value> {
        let data = json!({
        "id": query.id,    
        });
        create_request(
            "/api/dj/program/detail",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_program_detail, DjProgram);


// // 电台节目详情
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//   }
//   return request(`/api/dj/program/detail`, data, createOption(query, 'weapi'))
// }
