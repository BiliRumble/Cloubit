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


// TA关注的人(关注)
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/follows").route(web::get().to(user_follows)));
}

// 入参
define_request_struct!(UserFollows, {
    uid: String,
    offset: Option<u32>,
    limit: Option<u32>,
});

impl UserFollows {
    async fn requests(req: HttpRequest, query: Query<UserFollows>) -> Result<Response, Value> {
        let data = json!({
            "offset": query.offset.unwrap_or(0),
            "limit": query.limit.unwrap_or(30),
            "order": true,
        });
        create_request(
            &format!("/api/user/getfollows/{}", query.uid),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(user_follows, UserFollows);



// // TA关注的人(关注)
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     offset: query.offset || 0,
//     limit: query.limit || 30,
//     order: true,
//   }
//   return request(
//     `/api/user/getfollows/${query.uid}`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }