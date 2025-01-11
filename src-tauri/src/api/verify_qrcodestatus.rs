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

// 验证二维码状态
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/verify/qrcodestatus").route(web::get().to(verify_qrcodestatus)));
}

// 入参
define_request_struct!(VerifyQrCodeStatus, {
    qr: String,
});

impl VerifyQrCodeStatus {
    async fn requests(
        req: HttpRequest,
        query: Query<VerifyQrCodeStatus>,
    ) -> Result<Response, Value> {
        let data = json!({
            "qrCode": query.qr,
        });
        create_request(
            "/api/frontrisk/verify/qrcodestatus",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}

cache_handler!(verify_qrcodestatus, VerifyQrCodeStatus);

// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   const data = {
//     qrCode: query.qr,
//   }
//   const res = await request(
//     `/api/frontrisk/verify/qrcodestatus`,
//     data,
//     createOption(query, 'weapi'),
//   )
//   return res
// }
