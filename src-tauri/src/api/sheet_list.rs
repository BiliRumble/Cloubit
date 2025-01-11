
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

// 乐谱列表
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/music/sheet/list/v1").route(web::get().to(sheet_list)));
}

define_request_struct!(SheetList, {
    id: String,
    ab_test: Option<String>,
});

impl SheetList {
    async fn requests(req: HttpRequest, query: Query<SheetList>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "abTest": query.ab_test.clone().unwrap_or_else(|| "b".to_string()),
        });
        create_request(
            "/api/music/sheet/list/v1",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}


cache_handler!(sheet_list, SheetList);




// // 乐谱列表
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     abTest: query.ab || 'b',
//   }
//   return request(`/api/music/sheet/list/v1`, data, createOption(query))
// }