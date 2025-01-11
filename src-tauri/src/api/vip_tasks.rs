
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

// 会员任务
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/vip/tasks").route(web::get().to(vip_tasks)));
}

// 入参
define_request_struct!(VipTasks, {});

impl VipTasks {
    async fn requests(req: HttpRequest, query: Query<VipTasks>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/vipnewcenter/app/level/task/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(vip_tasks, VipTasks);




// // 会员任务
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(
//     `/api/vipnewcenter/app/level/task/list`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }