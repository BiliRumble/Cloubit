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

// // 精选电台
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/sub").route(web::get().to(dj_sub)));
}

// 入参
define_request_struct!(DjSub, {
    rid: String,
    t: i32
});

impl DjSub {
    async fn requests(req: HttpRequest, query: Query<DjSub>) -> Result<Response, Value> {
        let t = if query.t == 1 { "sub" } else { "unsub" };
        let data = json!({
        "id": query.rid,
    });
        create_request(
            &format!("/api/djradio/{}", t),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_sub, DjSub);

// // 订阅与取消电台
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.t = query.t == 1 ? 'sub' : 'unsub'
//   const data = {
//     id: query.rid,
//   }
//   return request(`/api/djradio/${query.t}`, data, createOption(query, 'weapi'))
// }
