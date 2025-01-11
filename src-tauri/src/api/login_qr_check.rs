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

//检查登录二维码
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login/qr/check").route(web::get().to(login_qr_check)));
}

// 入参
define_request_struct!(LoginQrCheck, {
    key: String,
});

impl LoginQrCheck {
    async fn requests(req: HttpRequest, query: Query<LoginQrCheck>) -> Result<Response, Value> {
        let data = json!({
            "key": query.key,
            "type": 3,
        });

        // 发起请求
        let response = create_request(
            "/api/login/qrcode/client/login",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await;

        match response {
            Ok(mut result) => {
                // 使用 join 将 Vec<String> 转为单个字符串
                let cookie_str = result.cookie.as_ref().map(|cookies| cookies.join(";")).unwrap_or_default();
                let mut body = result.body.clone();
                if let Some(body_obj) = body.as_object_mut() {
                    body_obj.insert("cookie".to_string(), json!(cookie_str));
                }

                let response_body = json!({
                    "status": 200,
                    "body": body,
                    "cookie": cookie_str,
                });

                Ok(Response {
                    status: 200,
                    body: response_body,
                    cookie: result.cookie,
                })
            }
            Err(_) => Ok(Response {
                status: 200,
                body: json!({}),
                cookie: None,
            }),
        }
    }
}
cache_handler!(login_qr_check, LoginQrCheck);


// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   const data = {
//     key: query.key,
//     type: 3,
//   }
//   try {
//     let result = await request(
//       `/api/login/qrcode/client/login`,
//       data,
//       createOption(query),
//     )
//     result = {
//       status: 200,
//       body: {
//         ...result.body,
//         cookie: result.cookie.join(';'),
//       },
//       cookie: result.cookie,
//     }
//     return result
//   } catch (error) {
//     return {
//       status: 200,
//       body: {},
//       cookie: result.cookie,
//     }
//   }
// }
