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

// 排行榜
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/top/list").route(web::get().to(top_list)));
}

// 入参
define_request_struct!(TopList, {
    id: String,
});

impl TopList {
    async fn requests(req: HttpRequest, query: Query<TopList>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "n": "500",
            "s": "0",
        });
        create_request(
            "/api/playlist/v4/detail",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(top_list, TopList);


// // 排行榜
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   if (query.idx) {
//     return Promise.resolve({
//       status: 500,
//       body: {
//         code: 500,
//         msg: '不支持此方式调用,只支持id调用',
//       },
//     })
//   }
// 
//   const data = {
//     id: query.id,
//     n: '500',
//     s: '0',
//   }
//   return request(`/api/playlist/v4/detail`, data, createOption(query))
// }