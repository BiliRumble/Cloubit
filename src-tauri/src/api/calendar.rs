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

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/calendar").route(web::get().to(calendar)));
}

// 入参
define_request_struct!(Calendar, {
    startTime: Option<i64>,
    endTime: Option<i64>,
});


impl Calendar {
    async fn requests(req: HttpRequest, query: Query<Calendar>) -> Result<Response, Value> {
        // 获取当前时间戳
        let current_timestamp = SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time went backwards")
            .as_millis() as i64;

        // 使用 `query.startTime` 和 `query.endTime`，如果不存在则使用当前时间戳作为默认值
        let data = json!({
            "startTime": query.startTime.unwrap_or(current_timestamp),
            "endTime": query.endTime.unwrap_or(current_timestamp),
        });

        // 发起请求
        create_request(
            "/api/mcalendar/detail",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(calendar, Calendar);


// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     startTime: query.startTime || Date.now(),
//     endTime: query.endTime || Date.now(),
//   }
//   return request(`/api/mcalendar/detail`, data, createOption(query, 'weapi'))
// }
