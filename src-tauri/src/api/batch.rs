use std::collections::HashMap;
use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Map, Value};
use std::str::FromStr;
use web::Query;

// // 批量请求接口
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/batch").route(web::get().to(batch)));
}

// 入参
define_request_struct!(Batch, {
    query_params: HashMap<String, String>
});

impl Batch {
  async fn requests(req: HttpRequest, query: Query<Batch>) -> Result<Response, Value> {
    // 初始化 data 为 JSON 对象
    let mut data = Map::new();

    // 遍历 query 中的参数，将以 "/api/" 开头的键值对添加到 data 中
    for (key, value) in &query.query_params {
      if key.starts_with("/api/") {
        data.insert(key.clone(), Value::String(value.clone()));
      }
    }

    // 将 data 转换为 JSON
    let data_json = Value::Object(data);

    // 发送请求
    create_request(
      "/api/batch",
      data_json,
      create_request_option(extract_headers!(req), &query.common, ""),
    ).await
  }
}
cache_handler!(batch, Batch);


// // 批量请求接口
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   Object.keys(query).forEach((i) => {
//     if (/^\/api\//.test(i)) {
//       data[i] = query[i]
//     }
//   })
//   return request(`/api/batch`, data, createOption(query))
// }
