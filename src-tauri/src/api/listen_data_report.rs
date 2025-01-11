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

// // 听歌足迹 - 周/月/年收听报告
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/listen/data/report").route(web::get().to(listen_data_report)));
}

// 入参
define_request_struct!(ListenDataReport, {
    r#type: Option<String>,
    endTime: String,
});

impl ListenDataReport {
    async fn requests(req: HttpRequest, query: Query<ListenDataReport>) -> Result<Response, Value> {
        let data = json!({
        "type": query.r#type.as_deref().unwrap_or("week"),
        "endTime": query.endTime,
    });
        create_request(
            "/api/content/activity/listen/data/report",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(listen_data_report, ListenDataReport);


// // 听歌足迹 - 周/月/年收听报告
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(
//     `/api/content/activity/listen/data/report`,
//     {
//       type: query.type || 'week', //周 week 月 month 年 year
//       endTime: query.endTime, // 不填就是本周/月的
//     },
//     createOption(query),
//   )
// }
