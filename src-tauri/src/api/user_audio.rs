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


// 用户创建的电台
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/audio").route(web::get().to(user_audio)));
}

// 入参
define_request_struct!(UserAudio, {
    uid: String,
});

impl UserAudio {
    async fn requests(req: HttpRequest, query: Query<UserAudio>) -> Result<Response, Value> {
        let data = json!({
            "userId": query.uid,
        });
        create_request(
            "/api/djradio/get/byuser",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(user_audio, UserAudio);


// // 用户创建的电台
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     userId: query.uid,
//   }
//   return request(`/api/djradio/get/byuser`, data, createOption(query, 'weapi'))
// }