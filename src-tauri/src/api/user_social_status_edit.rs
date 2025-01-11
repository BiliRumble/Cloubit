

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
    cfg.service(web::resource("/social/user/status/edit").route(web::get().to(user_social_status_edit)));
}

define_request_struct!(UserSocialStatusEdit, {
    r#type: String,
    icon_url: String,
    content: String,
    action_url: String,
});

impl UserSocialStatusEdit {
    async fn requests(req: HttpRequest, query: Query<UserSocialStatusEdit>) -> Result<Response, Value> {
        let data = json!({
            "type": query.r#type,
            "iconUrl": query.icon_url,
            "content": query.content,
            "actionUrl": query.action_url,
        });
        create_request(
            "/api/social/user/status/edit",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(user_social_status_edit, UserSocialStatusEdit);

// // 用户状态 - 编辑
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(
//     `/api/social/user/status/edit`,
//     {
//       content: JSON.stringify({
//         type: query.type,
//         iconUrl: query.iconUrl,
//         content: query.content,
//         actionUrl: query.actionUrl,
//       }),
//     },
//     createOption(query),
//   )
// }