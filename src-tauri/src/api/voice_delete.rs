
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

// 删除音频
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/voice/delete").route(web::post().to(voice_delete)));
}

// 入参
define_request_struct!(VoiceDelete, {
    ids: Vec<String>
});

impl VoiceDelete {
    async fn requests(req: HttpRequest, query: Query<VoiceDelete>) -> Result<Response, Value> {
        let data = json!({
            "ids": query.ids
        });
        create_request(
            "/api/content/voice/delete",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(voice_delete, VoiceDelete);



// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     ids: query.ids,
//   }
//   return request('/api/content/voice/delete', data, createOption(query))
// }