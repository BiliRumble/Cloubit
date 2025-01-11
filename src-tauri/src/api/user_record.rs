
use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/record").route(web::get().to(user_record)));
}

define_request_struct!(UserRecord, {
    uid: String,
    r#type: Option<i32>,
});

impl UserRecord {
    async fn requests(req: HttpRequest, query: Query<UserRecord>) -> Result<Response, Value> {
        let data = json!({
            "uid": query.uid,
            "type": query.r#type.unwrap_or(0),
        });
        create_request(
            "/api/v1/play/record",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}


cache_handler!(user_record, UserRecord);

// // 听歌排行
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     uid: query.uid,
//     type: query.type || 0, // 1: 最近一周, 0: 所有时间
//   }
//   return request(`/api/v1/play/record`, data, createOption(query, 'weapi'))
// }