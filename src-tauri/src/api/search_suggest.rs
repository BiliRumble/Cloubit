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

// 搜索建议
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/search/suggest").route(web::post().to(search_suggest)));
}

// 入参
define_request_struct!(SearchSuggest, {
    keywords: Option<String>,
    r#type: Option<String>,
});

impl SearchSuggest {
    async fn requests(req: HttpRequest, query: Query<SearchSuggest>) -> Result<Response, Value> {
        let data = json!({
            "s": query.keywords.clone().unwrap_or_default(),
        });
        let r#type = if query.r#type.as_deref() == Some("mobile") {
            "keyword"
        } else {
            "web"
        };
        create_request(
            &format!("/api/search/suggest/{}", r#type),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}
cache_handler!(search_suggest, SearchSuggest);

// // 搜索建议
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     s: query.keywords || '',
//   }
//   let type = query.type == 'mobile' ? 'keyword' : 'web'
//   return request(
//     `/api/search/suggest/` + type,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
