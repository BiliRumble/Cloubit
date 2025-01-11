use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use qrcode::render::unicode;
use qrcode::QrCode;
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use web::Query;

//创建二维码
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/login/qr/create").route(web::get().to(login_qr_create)));
}

// 入参
define_request_struct!(LoginQrCreate, {
    key: String,
    qrimg: Option<bool>,
});

impl LoginQrCreate {
    async fn requests(req: HttpRequest, query: Query<LoginQrCreate>) -> Result<Response, Value> {
        // 构建二维码 URL
        let url = format!("https://music.163.com/login?codekey={}", query.key);

        // 生成二维码图片的 Base64 数据（如果 `qrimg` 参数为 `true`）
        let qrimg = if query.qrimg.unwrap_or(false) {
            match QrCode::new(&url) {
                Ok(code) => {
                    let image = code.render::<unicode::Dense1x2>().build();
                    Some(image)
                }
                Err(_) => None,
            }
        } else {
            None
        };

        // 构建响应数据
        let response_body = json!({
            "code": 200,
            "data": {
                "qrurl": url,
                "qrimg": qrimg.unwrap_or_default(),
            },
        });

        Ok(Response {
            status: 200,
            body: response_body,
            cookie: None,
        })
    }
}
cache_handler!(login_qr_create, LoginQrCreate);


// const QRCode = require('qrcode')
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return new Promise(async (resolve) => {
//     const url = `https://music.163.com/login?codekey=${query.key}`
//     return resolve({
//       code: 200,
//       status: 200,
//       body: {
//         code: 200,
//         data: {
//           qrurl: url,
//           qrimg: query.qrimg ? await QRCode.toDataURL(url) : '',
//         },
//       },
//     })
//   })
// }
