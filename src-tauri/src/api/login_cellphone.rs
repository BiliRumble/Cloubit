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

// // 手机登录
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/login/cellphone").route(web::get().to(login_cellphone)));
}

// 入参
define_request_struct!(LoginCellphone, {
    phone: String,
    password: Option<String>,
    md5_password: Option<String>,
    countrycode: Option<String>,
    captcha: Option<String>,
});

impl LoginCellphone {
  async fn requests(req: HttpRequest, query: Query<LoginCellphone>) -> Result<Response, Value> {
    // 计算 MD5 密码
    let password = query.captcha.clone().unwrap_or_else(|| {
      query.md5_password.clone().unwrap_or_else(|| {
        format!("{:x}", md5::compute(query.password.clone().unwrap_or_default()))
      })
    });

    let data = json!({
            "type": "1",
            "https": "true",
            "phone": query.phone,
            "countrycode": query.countrycode.clone().unwrap_or_else(|| "86".to_string()),
            "captcha": query.captcha,
            if query.captcha.is_some() { "captcha" } else { "password" }: password,
            "rememberLogin": "true",
        });

    let response = create_request(
      "/api/w/login/cellphone",
      data,
      create_request_option(extract_headers!(req), &query.common, ""),
    ).await;

    match response {
      Ok(mut response) => {
        let code = response.body.get("code").and_then(|c| c.as_i64()).unwrap_or(0);

        if code == 200 {
          // 替换 avatarImgId_str 为 avatarImgIdStr
          if let Ok(body_str) = serde_json::to_string(&response.body) {
            let replaced_body = body_str.replace("avatarImgId_str", "avatarImgIdStr");
            response.body = serde_json::from_str(&replaced_body).unwrap_or(response.body);
          }

          Ok(Response {
            status: 200,
            body: json!({
                            "msg": "登录成功",
                            "code": 200,
                            "data": response.body,
                        }),
            cookie: response.cookie,
          })
        } else {
          Ok(Response {
            status: 400,
            body: json!({
                            "msg": "登录失败",
                            "code": code,
                            "message": response.body.get("message").unwrap_or(&Value::String("登录失败".into())),
                        }),
            cookie: None,
          })
        }
      }
      Err(e) => Err(e),
    }
  }
}
cache_handler!(login_cellphone, LoginCellphone);


// // 手机登录
// 
// const CryptoJS = require('crypto-js')
// 
// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   const data = {
//     type: '1',
//     https: 'true',
//     phone: query.phone,
//     countrycode: query.countrycode || '86',
//     captcha: query.captcha,
//     [query.captcha ? 'captcha' : 'password']: query.captcha
//       ? query.captcha
//       : query.md5_password || CryptoJS.MD5(query.password).toString(),
//     rememberLogin: 'true',
//   }
//   let result = await request(
//     `/api/w/login/cellphone`,
//     data,
//     createOption(query),
//   )
// 
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
