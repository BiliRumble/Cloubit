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

//获取登录二维码key
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login/qr/key").route(web::get().to(login_qr_key)));
}

// 入参
define_request_struct!(LoginQrKey, {
    key: String,
    qrimg: Option<bool>,
});

impl LoginQrKey {
    async fn requests(req: HttpRequest, query: Query<LoginQrKey>) -> Result<Response, Value> {
        // 构建请求数据
        let data = json!({
            "type": 3,
        });

        // 发送请求并处理响应
        let response = create_request(
            "/api/login/qrcode/unikey",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await;

        // 处理响应结果
        match response {
            Ok(mut response) => {
                Ok(Response {
                    status: 200,
                    body: json!({
                        "data": response.body,
                        "code": 200,
                    }),
                    cookie: response.cookie,
                })
            }
            Err(e) => Err(e),
        }
    }
}
cache_handler!(login_qr_key, LoginQrKey);

// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   const data = {
//     type: 3,
//   }
//   const result = await request(
//     `/api/login/qrcode/unikey`,
//     data,
//     createOption(query),
//   )
//   return {
//     status: 200,
//     body: {
//       data: result.body,
//       code: 200,
//     },
//     cookie: result.cookie,
//   }
// }
