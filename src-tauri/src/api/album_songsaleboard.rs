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

// 数字专辑&数字单曲-榜单
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/album/songsaleboard").route(web::get().to(album_songsaleboard)));
}

// 入参
define_request_struct!(AlbumSongsaleboard, {
    albumType: Option<String>,
    r#type: Option<String>,
    year: Option<String>,
});


impl AlbumSongsaleboard {
    async fn requests(req: HttpRequest, query: Query<AlbumSongsaleboard>) -> Result<Response, Value> {
        // 设置默认的 type 参数
        let query_type = query.r#type.clone().unwrap_or_else(|| "daily".to_string());

        // 构建 data 对象
        let mut data = json!({
            "albumType": query.albumType.clone().unwrap_or("0".to_string()), // 0为数字专辑,1为数字单曲
        });

        // 如果 type 为 "year"，添加 year 参数
        if query_type == "year" {
            if let Some(year) = &query.year {
                data["year"] = json!(year);
            }
        }

        // 动态构建 URL 路径
        let url = format!("/api/feealbum/songsaleboard/{}/type", query_type);

        // 执行请求
        create_request(
            &url,
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
            .await
    }
}
cache_handler!(album_songsaleboard, AlbumSongsaleboard);


// // 数字专辑&数字单曲-榜单
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   let data = {
//     albumType: query.albumType || 0, //0为数字专辑,1为数字单曲
//   }
//   const type = query.type || 'daily' // daily,week,year,total
//   if (type === 'year') {
//     data = {
//       ...data,
//       year: query.year,
//     }
//   }
//   return request(
//     `/api/feealbum/songsaleboard/${type}/type`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
