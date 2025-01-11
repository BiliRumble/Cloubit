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

// 歌曲链接
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/song/url").route(web::get().to(song_url)));
}

// 入参
define_request_struct!(SongUrl, {
    id: String,
    br: Option<i32>,
});

impl SongUrl {
    async fn requests(req: HttpRequest, query: Query<SongUrl>) -> Result<Response, Value> {
        let ids: Vec<&str> = query.id.split(',').collect();
        let data = json!({
            "ids": serde_json::to_string(&ids).unwrap(),
            "br": query.br.unwrap_or(999000),
        });
        let res = create_request(
            "/api/song/enhance/player/url",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        )
        .await?;
        let mut result = res.body["data"].as_array().unwrap().clone();
        result.sort_by_key(|a| {
            ids.iter()
                .position(|&id| id == a["id"].as_str().unwrap())
                .unwrap()
        });
        Ok(Response {
            status: 200,
            body: json!({
                "code": 200,
                "data": result,
            }),
            cookie: None,
        })
    }
}
cache_handler!(song_url, SongUrl);

// // 歌曲链接
// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   const ids = String(query.id).split(',')
//   const data = {
//     ids: JSON.stringify(ids),
//     br: parseInt(query.br || 999000),
//   }
//   const res = await request(
//     `/api/song/enhance/player/url`,
//     data,
//     createOption(query),
//   )
//   // 根据id排序
//   const result = res.body.data
//   result.sort((a, b) => {
//     return ids.indexOf(String(a.id)) - ids.indexOf(String(b.id))
//   })
//   return {
//     status: 200,
//     body: {
//       code: 200,
//       data: result,
//     },
//   }
// }
