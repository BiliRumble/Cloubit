use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use qrcode::render::svg;
use qrcode::QrCode;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use web::Query;

// 获取二维码
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/verify/getqrcode").route(web::get().to(verify_get_qr)));
}

// 入参
define_request_struct!(VerifyGetQr, {
    vid: String,
    r#type: String,
    token: String,
    evid: String,
    sign: String,
});

impl VerifyGetQr {
    async fn requests(req: HttpRequest, query: Query<VerifyGetQr>) -> Result<Response, Value> {
        let data = json!({
            "verifyConfigId": query.vid,
            "verifyType": query.r#type,
            "token": query.token,
            "params": json!({
                "event_id": query.evid,
                "sign": query.sign,
            }).to_string(),
            "size": 150,
        });

        let res = create_request(
            "/api/frontrisk/verify/getqrcode",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await?;

        let result = format!(
            "https://st.music.163.com/encrypt-pages?qrCode={}&verifyToken={}&verifyId={}&verifyType={}&params={}",
            res.body.get("qrCode").unwrap().to_string(),
            query.token,
            query.vid,
            query.r#type,
            json!({
                "event_id": query.evid,
                "sign": query.sign,
            })
        );

        let qr_code = QrCode::new(result.as_bytes()).unwrap();
        let qr_img = qr_code.render::<svg::Color>().build();

        Ok(Response {
            status: 200,
            body: json!({
                "code": 200,
                "data": {
                    "qrCode": res.body.get("qrCode").unwrap().to_string(),
                    "qrurl": result,
                    "qrimg": qr_img,
                },
            }),
            cookie: None,
        })
    }
}

cache_handler!(verify_get_qr, VerifyGetQr);

// const QRCode = require('qrcode')
// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   const data = {
//     verifyConfigId: query.vid,
//     verifyType: query.type,
//     token: query.token,
//     params: JSON.stringify({
//       event_id: query.evid,
//       sign: query.sign,
//     }),
//     size: 150,
//   }
//
//   const res = await request(
//     `/api/frontrisk/verify/getqrcode`,
//     data,
//     createOption(query, 'weapi'),
//   )
//   const result = `https://st.music.163.com/encrypt-pages?qrCode=${
//     res.body.data.qrCode
//   }&verifyToken=${query.token}&verifyId=${query.vid}&verifyType=${
//     query.type
//   }&params=${JSON.stringify({
//     event_id: query.evid,
//     sign: query.sign,
//   })}`
//   return {
//     status: 200,
//     body: {
//       code: 200,
//       data: {
//         qrCode: res.body.data.qrCode,
//         qrurl: result,
//         qrimg: await QRCode.toDataURL(result),
//       },
//     },
//   }
// }