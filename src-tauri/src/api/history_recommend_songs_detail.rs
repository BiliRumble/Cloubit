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

// // 历史每日推荐歌曲详情
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/history/recommend/songs").route(web::get().to(history_recommend_songs_detail)));
}

// 入参
define_request_struct!(HistoryRecommendSongsDetail, {
  date: Option<String>
});

impl HistoryRecommendSongsDetail {
    async fn requests(req: HttpRequest, query: Query<HistoryRecommendSongsDetail>) -> Result<Response, Value> {
        let data = json!({
        "date": query.date.as_ref().unwrap_or(&"".to_string()),
    });
        create_request(
            "/api/discovery/recommend/songs/history/detail",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(history_recommend_songs_detail, HistoryRecommendSongsDetail);


// // 历史每日推荐歌曲详情
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     date: query.date || '',
//   }
//   return request(
//     `/api/discovery/recommend/songs/history/detail`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
