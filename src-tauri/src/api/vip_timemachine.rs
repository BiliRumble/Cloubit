use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

// 黑胶时光机
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/vip/timemachine").route(web::get().to(vip_timemachine)));
}

// 入参
define_request_struct!(VipTimemachine, {
    start_time: Option<i64>,
    end_time: Option<i64>,
    limit: Option<i32>
});

impl VipTimemachine {
    async fn requests(req: HttpRequest, query: Query<VipTimemachine>) -> Result<Response, Value> {
        let mut data = json!({});
        if let (Some(start_time), Some(end_time)) = (query.start_time, query.end_time) {
            data["startTime"] = json!(start_time);
            data["endTime"] = json!(end_time);
            data["type"] = json!(1);
            data["limit"] = json!(query.limit.unwrap_or(60));
        }
        create_request(
            "/api/vipmusic/newrecord/weekflow",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(vip_timemachine, VipTimemachine);



// // 黑胶时光机
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   if (query.startTime && query.endTime) {
//     data.startTime = query.startTime
//     data.endTime = query.endTime
//     data.type = 1
//     data.limit = query.limit || 60
//   }
//   return request(
//     `/api/vipmusic/newrecord/weekflow`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }