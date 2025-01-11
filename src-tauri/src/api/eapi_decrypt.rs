use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::crypto::{eapi_req_decrypt, eapi_res_decrypt};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use web::Query;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/eapi/decrypt").route(web::get().to(eapi_decrypt)));
}

// 入参
define_request_struct!(EapiDecrypt, {
    hexString: Option<String>,
    isReq: Option<String>
});

impl EapiDecrypt {
    async fn requests(req: HttpRequest, query: Query<EapiDecrypt>) -> Result<Response, Value> {
        let hex_string = match &query.hexString {
            Some(s) => s.replace(" ", ""), // 去除空格
            None => {
                return Ok(Response {
                    status: 400,
                    body: json!({
                        "code": 400,
                        "message": "hex string is required"
                    }),
                    cookie: None,
                });
            }
        };


        // 根据 is_req 参数确定解密类型
        let is_req = query.isReq.as_deref() != Some("false"); // 默认是请求解密


        // 执行解密
        let data = {
            // 定义解密闭包，根据返回类型分别处理
            let decrypt = |decrypt_fn: fn(&str) -> Result<Value, Box<dyn std::error::Error>>| {
                decrypt_fn(&hex_string).unwrap_or_else(|_| json!(""))
            };
            if is_req {
                // 使用 eapi_req_decrypt 函数解密，返回类型为 Result<(String, Value), _>
                eapi_req_decrypt(&hex_string).map(|result| result.1).unwrap_or_else(|_| json!(""))
            } else {
                // 使用 eapi_res_decrypt 函数解密，直接返回 Value 类型
                decrypt(eapi_res_decrypt)
            }
        };

        // 返回解密后的数据
        Ok(Response {
            status: 200,
            body: json!({
                "code": 200,
                "data": data
            }),
            cookie: None,
        })
    }
}
cache_handler!(eapi_decrypt, EapiDecrypt);


// const { eapiResDecrypt, eapiReqDecrypt } = require('../util/crypto')
// 
// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   const hexString = query.hexString
//   const isReq = query.isReq != 'false'
//   if (!hexString) {
//     return {
//       status: 400,
//       body: {
//         code: 400,
//         message: 'hex string is required',
//       },
//     }
//   }
//   // 去除空格
//   let pureHexString = hexString.replace(/\s/g, '')
//   return {
//     status: 200,
//     body: {
//       code: 200,
//       data: isReq
//         ? eapiReqDecrypt(pureHexString)
//         : eapiResDecrypt(pureHexString),
//     },
//   }
// }
