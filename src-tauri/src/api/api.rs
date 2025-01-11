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
use crate::util::text::cookie_string_to_json;


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/api").route(web::get().to(api)));
}

// 入参
define_request_struct!(Api, {
    uri: String,
    data: String,
});


impl Api {
    async fn requests(req: HttpRequest, query: Query<Api>) -> Result<Response, Value> {
        let uri = query.uri.clone();
        // 尝试解析 `query.data` 为 JSON 对象
        let data = match serde_json::from_str::<Value>(&query.data) {
            Ok(parsed_data) => {
                // 检查 `data.cookie` 是否为字符串
                if let Some(cookie_str) = parsed_data.get("cookie").and_then(|v| v.as_str()) {
                    let cookie_json = cookie_string_to_json(cookie_str);
                    let mut modified_data = parsed_data.clone();
                    modified_data["cookie"] = json!(cookie_json);
                    modified_data
                } else {
                    parsed_data
                }
            }
            Err(_) => json!({}), // 如果解析失败，使用空对象
        };

        let crypto = query.common.crypto.clone().unwrap_or_default();
        create_request(
            &uri,
            data,
            create_request_option(extract_headers!(req), &query.common, &crypto),
        ).await
    }
}
cache_handler!(api, Api);


// const { cookieToJson } = require('../util/index')
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const uri = query.uri
//   let data = {}
//   try {
//     data =
//       typeof query.data === 'string' ? JSON.parse(query.data) : query.data || {}
//     if (typeof data.cookie === 'string') {
//       data.cookie = cookieToJson(data.cookie)
//       query.cookie = data.cookie
//     }
//   } catch (e) {
//     data = {}
//   }
// 
//   const crypto = query.crypto || ''
// 
//   const res = request(uri, data, createOption(query, crypto))
//   return res
// }
