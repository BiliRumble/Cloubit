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

// // 类别热门电台
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/radio/hot").route(web::get().to(dj_radio_hot)));
}

// 入参
define_request_struct!(DjRadioHot, {
    cateId: i32,
    limit: Option<i32>,
    offset: Option<i32>,
});

impl DjRadioHot {
    async fn requests(req: HttpRequest, query: Query<DjRadioHot>) -> Result<Response, Value> {
        let data = json!({
        "cateId": query.cateId,
        "limit": query.limit.unwrap_or(30),
        "offset": query.offset.unwrap_or(0),
        });
        create_request(
            "/api/djradio/hot",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_radio_hot, DjRadioHot);


// // 类别热门电台
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     cateId: query.cateId,
//     limit: query.limit || 30,
//     offset: query.offset || 0,
//   }
//   return request(`/api/djradio/hot`, data, createOption(query, 'weapi'))
// }
