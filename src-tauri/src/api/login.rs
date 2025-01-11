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

// // 邮箱登录
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login").route(web::get().to(login)));
}

// 入参
define_request_struct!(Login, {
    email: String,
    password: String,
    md5_password: Option<String>,
});

impl Login {
    async fn requests(req: HttpRequest, query: Query<Login>) -> Result<Response, Value> {
        let password_hash = query.md5_password.clone().unwrap_or_else(|| {
            format!("{:x}", md5::compute(query.password.clone()))
        });

        let data = json!({
            "type": "0",
            "https": "true",
            "username": query.email,
            "password": password_hash,
            "rememberLogin": "true",
        });

        let response = create_request(
            "/api/w/login",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await;

        match response {
            Ok(mut response) => {
                let code = response.body.get("code").and_then(|c| c.as_i64()).unwrap_or(0);
                if code == 502 {
                    return Ok(Response {
                        status: 200,
                        body: json!({
                            "msg": "账号或密码错误",
                            "code": 502,
                            "message": "账号或密码错误",
                        }),
                        cookie: None,
                    });
                } else if code == 200 {
                    // 替换 avatarImgId_str 为 avatarImgIdStr
                    if let Ok(body_str) = serde_json::to_string(&response.body) {
                        let replaced_body = body_str.replace("avatarImgId_str", "avatarImgIdStr");
                        response.body = serde_json::from_str(&replaced_body).unwrap_or(response.body);
                    }
                    return Ok(Response {
                        status: 200,
                        body: json!({
                            "msg": "登录成功",
                            "code": 200,
                            "data": response.body,
                        }),
                        cookie: response.cookie,
                    });
                }
            }
            Err(e) => return Err(e),
        }

        // 默认错误响应
        Ok(Response {
            status: 500,
            body: json!({
                "msg": "登录失败",
                "code": 500,
                "message": "登录失败",
            }),
            cookie: None,
        })
    }
}
cache_handler!(login, Login);


// // 邮箱登录
// 
// const CryptoJS = require('crypto-js')
// 
// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   const data = {
//     type: '0',
//     https: 'true',
//     username: query.email,
//     password: query.md5_password || CryptoJS.MD5(query.password).toString(),
//     rememberLogin: 'true',
//   }
//   let result = await request(`/api/w/login`, data, createOption(query))
//   if (result.body.code === 502) {
//     return {
//       status: 200,
//       body: {
//         msg: '账号或密码错误',
//         code: 502,
//         message: '账号或密码错误',
//       },
//     }
//   }
//   if (result.body.code === 200) {
//     result = {
//       status: 200,
//       body: {
//         ...JSON.parse(
//           JSON.stringify(result.body).replace(
//             /avatarImgId_str/g,
//             'avatarImgIdStr',
//           ),
//         ),
//         cookie: result.cookie.join(';'),
//       },
//       cookie: result.cookie,
//     }
//   }
//   return result
// }
