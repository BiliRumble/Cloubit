
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

pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/medal/user/page").route(web::get().to(user_medal)));
}

define_request_struct!(UserMedal, {
    uid: String,
});

impl UserMedal {
    async fn requests(req: HttpRequest, query: Query<UserMedal>) -> Result<Response, Value> {
        let data = json!({
            "uid": query.uid,
        });
        create_request(
            "/api/medal/user/page",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(user_medal, UserMedal);




// // 用户徽章
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(
//     `/api/medal/user/page`,
//     {
//       uid: query.uid,
//     },
//     createOption(query),
//   )
// }