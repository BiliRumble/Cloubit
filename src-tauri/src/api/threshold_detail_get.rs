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


// 获取达人达标信息
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/threshold/detail/get").route(web::get().to(threshold_detail_get)));
}

// 入参
define_request_struct!(ThresholdDetailGet, {});

impl ThresholdDetailGet {
    async fn requests(req: HttpRequest, query: Query<ThresholdDetailGet>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/influencer/web/apply/threshold/detail/get",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(threshold_detail_get, ThresholdDetailGet);


// // 获取达人达标信息
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(
//     `/api/influencer/web/apply/threshold/detail/get`,
//     data,
//     createOption(query),
//   )
// }