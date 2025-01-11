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

// // 歌手分类
// 
// /* 
//     type 取值
//     1:男歌手
//     2:女歌手
//     3:乐队
// 
//     area 取值
//     -1:全部
//     7华语
//     96欧美
//     8:日本
//     16韩国
//     0:其他
// 
//     initial 取值 a-z/A-Z
// */
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/artist/list").route(web::get().to(artist_list)));
}

// 入参
define_request_struct!(ArtistList, {
    offset: Option<String>,
    limit: Option<String>,
    initial: Option<String>,
    area: String,
    r#type: Option<String>,
});


impl ArtistList {
    async fn requests(req: HttpRequest, query: Query<ArtistList>) -> Result<Response, Value> {
        // 处理 `initial` 参数
        let initial_value = if let Some(initial) = &query.initial {
            if initial.chars().all(|c| c.is_alphabetic()) {
                // 如果 `initial` 是字母，将其转换为大写并取 ASCII 码
                initial.to_uppercase().chars().next().unwrap() as u32
            } else {
                // 如果 `initial` 是数字字符串，尝试将其转换为数字
                initial.parse::<u32>().unwrap_or_default()
            }
        } else {
            0 // 默认值
        };
        let data = json!({
            "initial": initial_value,
            "limit": query.limit.clone().unwrap_or("30".to_string()),
            "offset": query.offset.clone().unwrap_or("0".to_string()),
            "area": query.area,
            "type": query.r#type.clone().unwrap_or("1".to_string()),
            "total": true,
        });
        create_request(
            "/api/v1/artist/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(artist_list, ArtistList);



// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     initial: isNaN(query.initial)
//       ? (query.initial || '').toUpperCase().charCodeAt() || undefined
//       : query.initial,
//     offset: query.offset || 0,
//     limit: query.limit || 30,
//     total: true,
//     type: query.type || '1',
//     area: query.area,
//   }
//   return request(`/api/v1/artist/list`, data, createOption(query, 'weapi'))
// }
