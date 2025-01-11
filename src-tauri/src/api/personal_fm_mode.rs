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


// 私人FM - 模式选择
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/personal_fm_mode").route(web::get().to(personal_fm_mode)));
}

// 入参
define_request_struct!(PersonalFmMode, {
    mode: String,
    submode: Option<String>,
    limit: Option<u32>,
});

impl PersonalFmMode {
    async fn requests(req: HttpRequest, query: Query<PersonalFmMode>) -> Result<Response, Value> {
        let data = json!({
            "mode": query.mode,
            "subMode": query.submode,
            "limit": query.limit.unwrap_or(3),
        });
        create_request(
            "/api/v1/radio/get",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(personal_fm_mode, PersonalFmMode);


// // 私人FM - 模式选择
// 
// // aidj, DEFAULT, FAMILIAR, EXPLORE, SCENE_RCMD ( EXERCISE, FOCUS, NIGHT_EMO  )
// // 来不及解释这几个了
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     mode: query.mode,
//     subMode: query.submode,
//     limit: query.limit || 3,
//   }
//   return request(`/api/v1/radio/get`, data, createOption(query))
// }