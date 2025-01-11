
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

// 推荐新歌
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/personalized/newsong").route(web::get().to(personalized_newsong)));
}

// 入参
define_request_struct!(PersonalizedNewsong, {
    limit: Option<u32>,
    areaId: Option<u32>,
});

impl PersonalizedNewsong {
    async fn requests(req: HttpRequest, query: Query<PersonalizedNewsong>) -> Result<Response, Value> {
        let data = json!({
            "type": "recommend",
            "limit": query.limit.unwrap_or(10),
            "areaId": query.areaId.unwrap_or(0),
        });
        create_request(
            "/api/personalized/newsong",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(personalized_newsong, PersonalizedNewsong);


// // 推荐新歌
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     type: 'recommend',
//     limit: query.limit || 10,
//     areaId: query.areaId || 0,
//   }
//   return request(
//     `/api/personalized/newsong`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }