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

// 分享歌曲到动态
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/share/resource").route(web::get().to(share_resource)));
}

define_request_struct!(ShareResource, {
    r#type: Option<String>,
    msg: Option<String>,
    id: Option<String>,
});

impl ShareResource {
    async fn requests(req: HttpRequest, query: Query<ShareResource>) -> Result<Response, Value> {
        let data = json!({
            "type": query.r#type.clone().unwrap_or_else(|| "song".to_string()),
            "msg": query.msg.clone().unwrap_or_else(|| "".to_string()),
            "id": query.id.clone().unwrap_or_else(|| "".to_string()),
        });
        create_request(
            "/api/share/friends/resource",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}

cache_handler!(share_resource, ShareResource);

// // 分享歌曲到动态
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     type: query.type || 'song', // song,playlist,mv,djprogram,djradio,noresource
//     msg: query.msg || '',
//     id: query.id || '',
//   }
//   return request(
//     `/api/share/friends/resource`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
