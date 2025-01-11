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

// 歌曲详情
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/song/detail").route(web::get().to(song_detail)));
}

// 入参
define_request_struct!(SongDetail, {
    ids: String,
});

impl SongDetail {
    async fn requests(req: HttpRequest, query: Query<SongDetail>) -> Result<Response, Value> {
        let ids: Vec<&str> = query.ids.split(',').collect();
        let c = ids
            .iter()
            .map(|id| format!(r#"{{"id":{}}}"#, id))
            .collect::<Vec<String>>()
            .join(",");
        let data = json!({
            "c": format!("[{}]", c),
        });
        create_request(
            "/api/v3/song/detail",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        )
        .await
    }
}

cache_handler!(song_detail, SongDetail);

// // 歌曲详情
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.ids = query.ids.split(/\s*,\s*/)
//   const data = {
//     c: '[' + query.ids.map((id) => '{"id":' + id + '}').join(',') + ']',
//   }
//   return request(`/api/v3/song/detail`, data, createOption(query, 'weapi'))
// }
