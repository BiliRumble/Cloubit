
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
    cfg.service(web::resource("/voicelist/trans").route(web::get().to(voicelist_trans)));
}

// 入参
define_request_struct!(VoicelistTrans, {
  limit: Option<String>,
  offset: Option<String>,
  radioId: Option<String>,
  programId: Option<String>,
  position: Option<String>
});


impl VoicelistTrans {
    async fn requests(req: HttpRequest, query: Query<VoicelistTrans>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.clone().unwrap_or("200".to_string()),
            "offset": query.offset.clone().unwrap_or("0".to_string()),
            "radioId": query.radioId.clone().unwrap_or_default(),
            "programId": query.programId.clone().unwrap_or("0".to_string()),
            "position": query.position.clone().unwrap_or("1".to_string())
        });
        create_request(
            "/api/voice/workbench/radio/program/trans",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(voicelist_trans, VoicelistTrans);



// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || '200', // 每页数量
//     offset: query.offset || '0', // 偏移量
//     radioId: query.radioId || null, // 电台id
//     programId: query.programId || '0', // 节目id
//     position: query.position || '1', // 排序编号
//   }
//   return request(
//     `/api/voice/workbench/radio/program/trans`,
//     data,
//     createOption(query),
//   )
// }