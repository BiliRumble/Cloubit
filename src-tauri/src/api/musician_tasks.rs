
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

// 获取音乐人任务
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/musician/tasks").route(web::get().to(musician_tasks)));
}

// 入参
define_request_struct!(MusicianTasks, {});

impl MusicianTasks {
    async fn requests(req: HttpRequest, query: Query<MusicianTasks>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/nmusician/workbench/mission/cycle/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(musician_tasks, MusicianTasks);



// // 获取音乐人任务
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(
//     `/api/nmusician/workbench/mission/cycle/list`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }