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

// 领取云豆
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/musician/cloudbean/obtain").route(web::get().to(musician_cloudbean_obtain)));
}

// 入参
define_request_struct!(MusicianCloudbeanObtain, {
    userMissionId: String,
    period: String,
});

impl MusicianCloudbeanObtain {
    async fn requests(req: HttpRequest, query: Query<MusicianCloudbeanObtain>) -> Result<Response, Value> {
        let data = json!({
            "userMissionId": query.userMissionId,
            "period": query.period,
        });
        create_request(
            "/api/nmusician/workbench/mission/reward/obtain/new",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(musician_cloudbean_obtain, MusicianCloudbeanObtain);


// // 领取云豆
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     userMissionId: query.id,
//     period: query.period,
//   }
//   return request(
//     `/api/nmusician/workbench/mission/reward/obtain/new`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }