use crate::util::request::{create_request, RequestOption, Response};
use crate::util::text::cloudmusic_dll_encode_id;
use crate::DEVICE_ID;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde_json::{json, Value};
use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request_option};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use std::str::FromStr;
use web::Query;
use crate::util::request::QueryOption;


// // 访客登录
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/register/anonimous").route(web::get().to(register_anonimous_http)));
}

// 入参
define_request_struct!(REGISTER_ANOIMOUS_HTTP, {});

impl REGISTER_ANOIMOUS_HTTP {
	async fn requests(req: HttpRequest, query: Query<REGISTER_ANOIMOUS_HTTP>) -> Result<Response, Value> {
		let option = RequestOption {
			crypto: Some("weapi".to_string()),
			cookie: None,
			ua: None,
			ip: None,
			real_ip: None,
			proxy: None,
			headers: None,
			e_r: None,
		};

		let device_id = DEVICE_ID.clone();
		let encoded_id = STANDARD.encode(format!("{} {}", device_id, cloudmusic_dll_encode_id(&device_id)));
		let data = json!({
			"username": encoded_id
		});

		let response = create_request("/api/register/anonimous", data, option).await;

		match response {
			Ok(result) => {
				// 使用 join 将 Vec<String> 转为单个字符串
                let cookie_str = result.cookie.as_ref().map(|cookies| cookies.join(";")).unwrap_or_else(|| "".to_string());
                let mut body = result.body.clone();
                if let Some(body_obj) = body.as_object_mut() {
                    body_obj.insert("cookie".to_string(), json!(cookie_str));
                }

                let response_body = json!({
                    "code": 200,
                    "data": body,
                    "cookie": cookie_str,
                });

                Ok(Response {
                    status: 200,
                    body: response_body,
                    cookie: result.cookie,
                })
			},
			Err(_) => Ok(Response {
                status: 500,
                body: json!({}),
                cookie: None,
            }),
		}
	}
}

cache_handler!(register_anonimous_http, REGISTER_ANOIMOUS_HTTP);

pub async fn register_anonimous() -> Result<Response, Value> {
    let option = RequestOption {
        crypto: Some("weapi".to_string()),
        cookie: None,
        ua: None,
        ip: None,
        real_ip: None,
        proxy: None,
        headers: None,
        e_r: None,
    };

    let device_id = DEVICE_ID.clone();
    let encoded_id = STANDARD.encode(format!("{} {}", device_id, cloudmusic_dll_encode_id(&device_id)));
    let data = json!({
        "username": encoded_id
    });

    // 调用异步函数，并直接返回
    create_request("/api/register/anonimous", data, option).await
}


// const CryptoJS = require('crypto-js')
// const path = require('path')
// const fs = require('fs')
// const ID_XOR_KEY_1 = '3go8&$8*3*3h0k(2)2'
// const deviceidText = fs.readFileSync(
//   path.resolve(__dirname, '../data/deviceid.txt'),
//   'utf-8',
// )
//
// const createOption = require('../util/option.js')
// const deviceidList = deviceidText.split('\n')
//
// function getRandomFromList(list) {
//   return list[Math.floor(Math.random() * list.length)]
// }
// function cloudmusic_dll_encode_id(some_id) {
//   let xoredString = ''
//   for (let i = 0; i < some_id.length; i++) {
//     const charCode =
//       some_id.charCodeAt(i) ^ ID_XOR_KEY_1.charCodeAt(i % ID_XOR_KEY_1.length)
//     xoredString += String.fromCharCode(charCode)
//   }
//   const wordArray = CryptoJS.enc.Utf8.parse(xoredString)
//   const digest = CryptoJS.MD5(wordArray)
//   return CryptoJS.enc.Base64.stringify(digest)
// }
//
// module.exports = async (query, request) => {
//   const deviceId = getRandomFromList(deviceidList)
//   global.deviceId = deviceId
//   const encodedId = CryptoJS.enc.Base64.stringify(
//     CryptoJS.enc.Utf8.parse(
//       `${deviceId} ${cloudmusic_dll_encode_id(deviceId)}`,
//     ),
//   )
//   const data = {
//     username: encodedId,
//   }
//   let result = await request(
//     `/api/register/anonimous`,
//     data,
//     createOption(query, 'weapi'),
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
