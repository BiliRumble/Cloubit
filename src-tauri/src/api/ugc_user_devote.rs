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

// 用户贡献条目、积分、云贝数量
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ugc/user/devote").route(web::get().to(ugc_user_devote)));
}

// 入参
define_request_struct!(UgcUserDevote, {});


impl UgcUserDevote {
    async fn requests(req: HttpRequest, query: Query<UgcUserDevote>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/rep/ugc/user/devote",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(ugc_user_devote, UgcUserDevote);



// // 用户贡献条目、积分、云贝数量
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(`/api/rep/ugc/user/devote`, data, createOption(query))
// }