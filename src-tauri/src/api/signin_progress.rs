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

// 签到进度
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/signin/progress").route(web::get().to(signin_progress)));
}

// 入参
define_request_struct!(SigninProgress, {
    module_id: Option<String>,
});

impl SigninProgress {
    async fn requests(req: HttpRequest, query: Query<SigninProgress>) -> Result<Response, Value> {
        let data = json!({
            "moduleId": query.module_id.clone().unwrap_or_else(|| "1207signin-1207signin".to_string()),
        });
        create_request(
            "/api/act/modules/signin/v2/progress",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}

cache_handler!(signin_progress, SigninProgress);

// // 签到进度
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     moduleId: query.moduleId || '1207signin-1207signin',
//   }
//   return request(
//     `/api/act/modules/signin/v2/progress`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
