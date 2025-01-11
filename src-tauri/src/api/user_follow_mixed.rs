
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

// 当前账号关注的用户/歌手
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/follow/mixed").route(web::get().to(user_follow_mixed)));
}

// 入参
define_request_struct!(UserFollowMixed, {
    size: Option<u32>,
    cursor: Option<u32>,
    scene: Option<u32>,
});

impl UserFollowMixed {
    async fn requests(req: HttpRequest, query: Query<UserFollowMixed>) -> Result<Response, Value> {
        let data = json!({
            "authority": "false",
            "page": json!({
                "size": query.size.unwrap_or(30),
                "cursor": query.cursor.unwrap_or(0),
            }),
            "scene": query.scene.unwrap_or(0),
            "size": query.size.unwrap_or(30),
            "sortType": "0",
        });
        create_request(
            "/api/user/follow/users/mixed/get/v2",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(user_follow_mixed, UserFollowMixed);



// // 当前账号关注的用户/歌手
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const size = query.size || 30
//   const cursor = query.cursor || 0
//   const scene = query.scene || 0 // 0: 所有关注 1: 关注的歌手 2: 关注的用户
//   const data = {
//     authority: 'false',
//     page: JSON.stringify({
//       size: size,
//       cursor: cursor,
//     }),
//     scene: scene,
//     size: size,
//     sortType: '0',
//   }
//   return request(
//     `/api/user/follow/users/mixed/get/v2`,
//     data,
//     createOption(query),
//   )
// }