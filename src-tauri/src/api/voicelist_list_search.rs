
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
    cfg.service(web::resource("/voicelist/list/search").route(web::get().to(voicelist_list_search)));
}

// 入参
define_request_struct!(VoicelistListSearch, {
  limit: Option<String>,
  offset: Option<String>,
  name: Option<String>,
  displayStatus: Option<String>,
  r#type: Option<String>,
  voiceFeeType: Option<String>,
  radioId: String
});


impl VoicelistListSearch {
    async fn requests(req: HttpRequest, query: Query<VoicelistListSearch>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.clone().unwrap_or("200".to_string()),
            "offset": query.offset.clone().unwrap_or("0".to_string()),
            "name": query.name.clone().unwrap_or_default(),
            "displayStatus": query.displayStatus.clone().unwrap_or_default(),
            "type": query.r#type.clone().unwrap_or_default(),
            "voiceFeeType": query.voiceFeeType.clone().unwrap_or_default(),
            "radioId": query.radioId.clone()
        });
        create_request(
            "/api/voice/workbench/voice/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(voicelist_list_search, VoicelistListSearch);


// //声音搜索
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || '200',
//     offset: query.offset || '0',
//     name: query.name || null,
//     displayStatus: query.displayStatus || null,
//     type: query.type || null,
//     voiceFeeType: query.voiceFeeType || null,
//     radioId: query.voiceListId,
//   }
//   return request('/api/voice/workbench/voice/list', data, createOption(query))
// }