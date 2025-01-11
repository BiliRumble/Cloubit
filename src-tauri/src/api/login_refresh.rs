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

// 登录刷新服务配置
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login/refresh").route(web::get().to(login_refresh)));
}

// 入参结构体定义（这里没有请求参数）
define_request_struct!(LoginRefresh, {});

impl LoginRefresh {
    async fn requests(req: HttpRequest, query: Query<LoginRefresh>) -> Result<Response, Value> {
        // 构建空请求数据
        let data = json!({});

        // 发送请求并处理响应
        let response = create_request(
            "/api/login/token/refresh",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await;

        // 处理响应结果
        match response {
            Ok(mut response) => {
                let code = response.body.get("code").and_then(|v| v.as_i64()).unwrap_or(0);
                if code == 200 {
                    // 直接使用 `cookie`，如果不存在则提供空字符串
                    let cookie_string = response.cookie.clone().unwrap_or_default();

                    Ok(Response {
                        status: 200,
                        body: json!({
                            "data": response.body,
                            "code": 200,
                            "cookie": cookie_string,
                        }),
                        cookie: response.cookie,
                    })
                } else {
                    Ok(Response {
                        status: 200,
                        body: response.body,
                        cookie: response.cookie,
                    })
                }
            }
            Err(e) => Err(e),
        }
    }
}

// 缓存处理器
cache_handler!(login_refresh, LoginRefresh);


// // 登录刷新
//
// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   let result = await request(
//     `/api/login/token/refresh`,
//     {},
//     createOption(query),
//   )
//   if (result.body.code === 200) {
//     result = {
//       status: 200,
//       body: {
//         ...result.body,
//         cookie: result.cookie.join(';'),
//       },
//       cookie: result.cookie,
//     }
//   }
//   return result
// }
