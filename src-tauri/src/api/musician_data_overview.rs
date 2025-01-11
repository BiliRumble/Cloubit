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

// 音乐人数据概况
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/musician/data/overview").route(web::get().to(musician_data_overview)));
}

// 入参
define_request_struct!(MusicianDataOverview, {});

impl MusicianDataOverview {
    async fn requests(req: HttpRequest, query: Query<MusicianDataOverview>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/creator/musician/statistic/data/overview/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(musician_data_overview, MusicianDataOverview);


// // 音乐人数据概况
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(
//     `/api/creator/musician/statistic/data/overview/get`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }