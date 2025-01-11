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


// // 电台个性推荐
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/personalize/recommend").route(web::get().to(dj_personalize_recommend)));
}

// 入参
define_request_struct!(DjPersonalizeRecommend, {
  limit: Option<i32>,
});

impl DjPersonalizeRecommend {
    async fn requests(req: HttpRequest, query: Query<DjPersonalizeRecommend>) -> Result<Response, Value> {
        let data = json!({
        "limit": query.limit.unwrap_or(6),
    });
        create_request(
            "/api/djradio/personalize/rcmd",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_personalize_recommend, DjPersonalizeRecommend);


// // 电台个性推荐
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(
//     `/api/djradio/personalize/rcmd`,
//     {
//       limit: query.limit || 6,
//     },
//     createOption(query, 'weapi'),
//   )
// }
