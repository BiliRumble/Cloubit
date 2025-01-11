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

// 音乐人歌曲播放趋势
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/musician/play/trend").route(web::get().to(musician_play_trend)));
}

// 入参
define_request_struct!(MusicianPlayTrend, {
    startTime: String,
    endTime: String,
});

impl MusicianPlayTrend {
    async fn requests(req: HttpRequest, query: Query<MusicianPlayTrend>) -> Result<Response, Value> {
        let data = json!({
            "startTime": query.startTime,
            "endTime": query.endTime,
        });
        create_request(
            "/api/creator/musician/play/count/statistic/data/trend/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(musician_play_trend, MusicianPlayTrend);



// // 音乐人歌曲播放趋势
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     startTime: query.startTime,
//     endTime: query.endTime,
//   }
//   return request(
//     `/api/creator/musician/play/count/statistic/data/trend/get`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }