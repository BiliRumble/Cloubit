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
use std::time::{SystemTime, UNIX_EPOCH};
use web::Query;

// // 粉丝来源
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/fanscenter/trend/list").route(web::get().to(fanscenter_trend_list)));
}

// 入参
define_request_struct!(FanscenterTrendList, {
  startTime: Option<i64>,
  endTime: Option<i64>,
  r#type: Option<i64>,
});

impl FanscenterTrendList {
    async fn requests(req: HttpRequest, query: Query<FanscenterTrendList>) -> Result<Response, Value> {
        // 获取当前时间戳
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64;

        let data = json!({
        "startTime": query.startTime.unwrap_or(current_timestamp - 7 * 24 * 3600 * 1000),
        "endTime": query.endTime.unwrap_or(current_timestamp),
        "type": query.r#type.unwrap_or(0),
    });
        create_request(
            "/api/fanscenter/trend/list",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(fanscenter_trend_list, FanscenterTrendList);


// // 粉丝来源
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     startTime: query.startTime || Date.now() - 7 * 24 * 3600 * 1000,
//     endTime: query.endTime || Date.now(),
//     type: query.type || 0, //新增关注:0 新增取关:1
//   }
//   return request(`/api/fanscenter/trend/list`, data, createOption(query))
// }
