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

// // 签到
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/daily/signin").route(web::get().to(daily_signin)));
}

// 入参
define_request_struct!(DailySignin, {
   r#type: Option<i64>
});

impl DailySignin {
    async fn requests(req: HttpRequest, query: Query<DailySignin>) -> Result<Response, Value> {
        let data = json!({
        "type": query.r#type.unwrap_or(0),
        });
        create_request(
            "/api/point/dailyTask",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(daily_signin, DailySignin);


// // 签到
// 
// /*
//     0为安卓端签到 3点经验, 1为网页签到,2点经验
//     签到成功 {'android': {'point': 3, 'code': 200}, 'web': {'point': 2, 'code': 200}}
//     重复签到 {'android': {'code': -2, 'msg': '重复签到'}, 'web': {'code': -2, 'msg': '重复签到'}}
//     未登录 {'android': {'code': 301}, 'web': {'code': 301}}
// */
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     type: query.type || 0,
//   }
//   return request(`/api/point/dailyTask`, data, createOption(query, 'weapi'))
// }
