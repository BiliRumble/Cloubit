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


// // 每日推荐歌单
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/recommend/resource").route(web::get().to(recommend_resource)));
}

// 入参
define_request_struct!(RecommendResource, {});

impl RecommendResource {
    async fn requests(req: HttpRequest, query: Query<RecommendResource>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/v1/discovery/recommend/resource",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(recommend_resource, RecommendResource);


// // 每日推荐歌单
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(
//     `/api/v1/discovery/recommend/resource`,
//     {},
//     createOption(query, 'weapi'),
//   )
// }