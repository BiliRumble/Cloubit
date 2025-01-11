
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

// 全部MV
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/mv/all").route(web::get().to(mv_all)));
}

// 入参
define_request_struct!(MvAll, {
    area: Option<String>,
    type_: Option<String>,
    order: Option<String>,
    offset: Option<u32>,
    limit: Option<u32>,
});

impl MvAll {
    async fn requests(req: HttpRequest, query: Query<MvAll>) -> Result<Response, Value> {
        let tags = json!({
            "地区": query.area.clone().unwrap_or_else(|| "全部".to_string()),
            "类型": query.type_.clone().unwrap_or_else(|| "全部".to_string()),
            "排序": query.order.clone().unwrap_or_else(|| "上升最快".to_string()),
        });
        let data = json!({
            "tags": tags.to_string(),
            "offset": query.offset.unwrap_or(0),
            "total": true,
            "limit": query.limit.unwrap_or(30),
        });
        create_request(
            "/api/mv/all",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(mv_all, MvAll);




// // 全部MV
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     tags: JSON.stringify({
//       地区: query.area || '全部',
//       类型: query.type || '全部',
//       排序: query.order || '上升最快',
//     }),
//     offset: query.offset || 0,
//     total: 'true',
//     limit: query.limit || 30,
//   }
//   return request(`/api/mv/all`, data, createOption(query))
// }