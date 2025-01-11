
use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

// 获取音频详情
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/voice/detail").route(web::get().to(voice_detail)));
}

// 入参
define_request_struct!(VoiceDetail, {
    id: String
});

impl VoiceDetail {
    async fn requests(req: HttpRequest, query: Query<VoiceDetail>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id
        });
        create_request(
            "/api/voice/workbench/voice/detail",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(voice_detail, VoiceDetail);


// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//   }
//   return request(`/api/voice/workbench/voice/detail`, data, createOption(query))
// }