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

// mlog链接
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/mlog/url").route(web::get().to(mlog_url)));
}

// 入参
define_request_struct!(MlogUrl, {
    id: String,
    res: Option<i32>,
});

impl MlogUrl {
    async fn requests(req: HttpRequest, query: Query<MlogUrl>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "resolution": query.res.unwrap_or(1080),
            "type": 1,
        });
        create_request(
            "/api/mlog/detail/v1",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(mlog_url, MlogUrl);



// // mlog链接
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     resolution: query.res || 1080,
//     type: 1,
//   }
//   return request(`/api/mlog/detail/v1`, data, createOption(query, 'weapi'))
// }