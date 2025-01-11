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
use chrono::Datelike;
use web::Query;

// 新碟上架
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/top/album").route(web::get().to(top_album)));
}

// 入参
define_request_struct!(TopAlbum, {
    area: Option<String>,
    limit: Option<u32>,
    offset: Option<u32>,
    r#type: Option<String>,
    year: Option<u32>,
    month: Option<u32>,
});


impl TopAlbum {
    async fn requests(req: HttpRequest, query: Query<TopAlbum>) -> Result<Response, Value> {
        let date = chrono::Utc::now();
        let data = json!({
            "area": query.area.clone().unwrap_or("ALL".to_string()),
            "limit": query.limit.unwrap_or(50),
            "offset": query.offset.unwrap_or(0),
            "type": query.r#type.clone().unwrap_or("new".to_string()),
            "year": query.year.unwrap_or(date.year() as u32),
            "month": query.month.unwrap_or(date.month()),
            "total": false,
            "rcmd": true,
        });
        create_request(
            "/api/discovery/new/albums/area",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(top_album, TopAlbum);



// // 新碟上架
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const date = new Date()
// 
//   const data = {
//     area: query.area || 'ALL', // //ALL:全部,ZH:华语,EA:欧美,KR:韩国,JP:日本
//     limit: query.limit || 50,
//     offset: query.offset || 0,
//     type: query.type || 'new',
//     year: query.year || date.getFullYear(),
//     month: query.month || date.getMonth() + 1,
//     total: false,
//     rcmd: true,
//   }
//   return request(
//     `/api/discovery/new/albums/area`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }