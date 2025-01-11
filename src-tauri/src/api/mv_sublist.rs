
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

// 已收藏MV列表
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/mv/sublist").route(web::get().to(mv_sublist)));
}

// 入参
define_request_struct!(MvSublist, {
    limit: Option<u32>,
    offset: Option<u32>,
});

impl MvSublist {
    async fn requests(req: HttpRequest, query: Query<MvSublist>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.unwrap_or(25),
            "offset": query.offset.unwrap_or(0),
            "total": true,
        });
        create_request(
            "/api/cloudvideo/allvideo/sublist",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(mv_sublist, MvSublist);


// // 已收藏MV列表
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 25,
//     offset: query.offset || 0,
//     total: true,
//   }
//   return request(
//     `/api/cloudvideo/allvideo/sublist`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }