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

// 私信
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/send/text").route(web::get().to(send_text)));
}

define_request_struct!(SendText, {
    msg: String,
    user_ids: String,
});

impl SendText {
    async fn requests(
        req: HttpRequest,
        query: Query<SendText>,
    ) -> Result<Response, Value> {
        let data = json!({
            "type": "text",
            "msg": query.msg,
            "userIds": format!("[{}]", query.user_ids),
        });
        create_request(
            "/api/msg/private/send",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}

cache_handler!(send_text, SendText);

// // 私信
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     type: 'text',
//     msg: query.msg,
//     userIds: '[' + query.user_ids + ']',
//   }
//   return request(`/api/msg/private/send`, data, createOption(query, 'weapi'))
// }
