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

// // 新晋电台榜/热门电台榜
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/toplist").route(web::get().to(dj_toplist)));
}

// 入参
define_request_struct!(DjToplist, {
    limit: Option<String>,
    offset: Option<String>,
    r#type: Option<String>,
});

impl DjToplist {
    async fn requests(req: HttpRequest, query: Query<DjToplist>) -> Result<Response, Value> {
        let data = json!({
        "limit": query.limit.as_deref().unwrap_or("100"),
        "offset": query.offset.as_deref().unwrap_or("0"),
        "type": match query.r#type.as_deref().unwrap_or("new") {
            "new" => 0,
            "hot" => 1,
            _ => 0,
        },
    });
        create_request(
            "/api/djradio/toplist",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_toplist, DjToplist);


// // 新晋电台榜/热门电台榜
// const typeMap = {
//   new: 0,
//   hot: 1,
// }
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 100,
//     offset: query.offset || 0,
//     type: typeMap[query.type || 'new'] || '0', //0为新晋,1为热门
//   }
//   return request(`/api/djradio/toplist`, data, createOption(query, 'weapi'))
// }
