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


// 昵称检查
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/nickname_check").route(web::get().to(nickname_check)));
}

// 入参
define_request_struct!(NicknameCheck, {
    nickname: String,
});

impl NicknameCheck {
    async fn requests(req: HttpRequest, query: Query<NicknameCheck>) -> Result<Response, Value> {
        let data = json!({
            "nickname": query.nickname,
        });
        create_request(
            "/api/nickname/duplicated",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(nickname_check, NicknameCheck);



// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     nickname: query.nickname,
//   }
//   return request(`/api/nickname/duplicated`, data, createOption(query, 'weapi'))
// }