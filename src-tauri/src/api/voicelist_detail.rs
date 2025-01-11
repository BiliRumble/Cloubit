
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

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/voicelist/detail").route(web::get().to(voicelist_detail)));
}

// 入参
define_request_struct!(VoicelistDetail, {
  id: String
});


impl VoicelistDetail {
    async fn requests(req: HttpRequest, query: Query<VoicelistDetail>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id.clone()
        });
        create_request(
            "/api/voice/workbench/voicelist/detail",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(voicelist_detail, VoicelistDetail);


// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//   }
//   return request(
//     `/api/voice/workbench/voicelist/detail`,
//     data,
//     createOption(query),
//   )
// }