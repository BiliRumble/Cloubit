
use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{from_str, json, Value};
use web::Query;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

// 用户详情
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/detail").route(web::get().to(user_detail)));
}

// 入参
define_request_struct!(UserDetail, {
    uid: String,
});

impl UserDetail {
    async fn requests(req: HttpRequest, query: Query<UserDetail>) -> Result<Response, Value> {
        let res = create_request(
            &format!("/api/v1/user/detail/{}", query.uid),
            json!({}),
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await?;
        // 将返回结果转换为字符串并替换字段名
        let result = res.body.to_string().replace("avatarImgId_str", "avatarImgIdStr");

        // 将字符串转换回 JSON 对象
        let json_result: Value = from_str(result.as_str()).unwrap();

        Ok(Response {
            status: 200,
            body: json_result,
            cookie: None,
        })
    }
}
cache_handler!(user_detail, UserDetail);


// // 用户详情
// 
// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   const res = await request(
//     `/api/v1/user/detail/${query.uid}`,
//     {},
//     createOption(query, 'weapi'),
//   )
//   const result = JSON.stringify(res).replace(
//     /avatarImgId_str/g,
//     'avatarImgIdStr',
//   )
//   return JSON.parse(result)
// }