
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

// 多类型搜索
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/search/multimatch").route(web::post().to(search_multimatch)));
}

// 入参
define_request_struct!(SearchMultimatch, {
    r#type: Option<String>,
    keywords: Option<String>,
});

impl SearchMultimatch {
    async fn requests(req: HttpRequest, query: Query<SearchMultimatch>) -> Result<Response, Value> {
        let data = json!({
            "type": query.r#type.clone().unwrap_or_else(|| "1".to_string()),
            "s": query.keywords.clone().unwrap_or_default(),
        });
        create_request(
            "/api/search/suggest/multimatch",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(search_multimatch, SearchMultimatch);


// // 多类型搜索
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     type: query.type || 1,
//     s: query.keywords || '',
//   }
//   return request(
//     `/api/search/suggest/multimatch`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }