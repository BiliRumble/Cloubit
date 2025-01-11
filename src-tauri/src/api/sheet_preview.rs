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

// 乐谱预览
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/music/sheet/preview/info").route(web::get().to(sheet_preview)));
}

define_request_struct!(SheetPreview, {
    id: String,
});

impl SheetPreview {
    async fn requests(req: HttpRequest, query: Query<SheetPreview>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
        });
        create_request(
            "/api/music/sheet/preview/info",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        )
        .await
    }
}

cache_handler!(sheet_preview, SheetPreview);

// // 乐谱预览
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//   }
//   return request(`/api/music/sheet/preview/info`, data, createOption(query))
// }
