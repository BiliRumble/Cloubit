
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

// 独家放送列表
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/personalized/privatecontent/list").route(web::get().to(personalized_privatecontent_list)));
}

// 入参
define_request_struct!(PersonalizedPrivatecontentList, {
    offset: Option<u32>,
    limit: Option<u32>,
});

impl PersonalizedPrivatecontentList {
    async fn requests(req: HttpRequest, query: Query<PersonalizedPrivatecontentList>) -> Result<Response, Value> {
        let data = json!({
            "offset": query.offset.unwrap_or(0),
            "total": "true",
            "limit": query.limit.unwrap_or(60),
        });
        create_request(
            "/api/v2/privatecontent/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(personalized_privatecontent_list, PersonalizedPrivatecontentList);



// // 独家放送列表
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     offset: query.offset || 0,
//     total: 'true',
//     limit: query.limit || 60,
//   }
//   return request(
//     `/api/v2/privatecontent/list`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }