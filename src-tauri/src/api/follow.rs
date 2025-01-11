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

// // 关注与取消关注用户
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/follow").route(web::get().to(follow)));
}

// 入参
define_request_struct!(Follow, {
    id: i32,
    t: i32
});

impl Follow {
    async fn requests(req: HttpRequest, query: Query<Follow>) -> Result<Response, Value> {
        let t = if query.t == 1 { "follow" } else { "delfollow" };
        let data = json!({});
        create_request(
            &format!("api/user/{}/{}", t, query.id),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(follow, Follow);


// // 关注与取消关注用户
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.t = query.t == 1 ? 'follow' : 'delfollow'
//   return request(
//     `/api/user/${query.t}/${query.id}`,
//     {},
//     createOption(query, 'weapi'),
//   )
// }
