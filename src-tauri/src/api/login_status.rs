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

// 配置服务：登录状态刷新
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login/status").route(web::get().to(login_status)));
}

// 请求参数结构体（此处没有参数）
define_request_struct!(LoginRefresh, {});

impl LoginRefresh {
    async fn requests(req: HttpRequest, query: Query<LoginRefresh>) -> Result<Response, Value> {
        // 构建请求数据（此处为空对象）
        let data = json!({});

        // 发起请求并获取响应
        let response = create_request(
            "/api/w/nuser/account/get",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await;

        // 处理响应
        match response {
            Ok(mut response) => {
                let code = response.body.get("code").and_then(|v| v.as_i64()).unwrap_or(0);
                if code == 200 {
                    // 当返回 code 为 200 时，构造成功响应
                    Ok(Response {
                        status: 200,
                        body: json!({
                            "data": response.body,
                        }),
                        cookie: response.cookie,
                    })
                } else {
                    // 返回其他响应
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
cache_handler!(login_status, LoginRefresh);


// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   const data = {}
//   let result = await request(
//     `/api/w/nuser/account/get`,
//     data,
//     createOption(query, 'weapi'),
//   )
//   if (result.body.code === 200) {
//     result = {
//       status: 200,
//       body: {
//         data: {
//           ...result.body,
//         },
//       },
//       cookie: result.cookie,
//     }
//   }
//   return result
// }
