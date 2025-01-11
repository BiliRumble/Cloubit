
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

// 编辑用户信息
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/profile/update").route(web::get().to(user_update)));
}

// 入参
define_request_struct!(UserUpdate, {
    birthday: String,
    city: String,
    gender: String,
    nickname: String,
    province: String,
    signature: String,
});

impl UserUpdate {
    async fn requests(req: HttpRequest, query: Query<UserUpdate>) -> Result<Response, Value> {
        let data = json!({
            "birthday": query.birthday,
            "city": query.city,
            "gender": query.gender,
            "nickname": query.nickname,
            "province": query.province,
            "signature": query.signature,
        });
        create_request(
            "/api/user/profile/update",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(user_update, UserUpdate);


// // 编辑用户信息
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     // avatarImgId: '0',
//     birthday: query.birthday,
//     city: query.city,
//     gender: query.gender,
//     nickname: query.nickname,
//     province: query.province,
//     signature: query.signature,
//   }
//   return request(`/api/user/profile/update`, data, createOption(query, 'weapi'))
// }