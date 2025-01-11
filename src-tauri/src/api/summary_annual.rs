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

// 年度听歌报告2017-2023
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/summary_annual").route(web::get().to(summary_annual)));
}

// 入参
define_request_struct!(SummaryAnnual, {
    year: String,
});

impl SummaryAnnual {
    async fn requests(req: HttpRequest, query: Query<SummaryAnnual>) -> Result<Response, Value> {
        let key = if ["2017", "2018", "2019"].contains(&query.year.as_str()) {
            "userdata"
        } else {
            "data"
        };
        let data = json!({});
        create_request(
            &format!("/api/activity/summary/annual/{}/{}", query.year, key),
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(summary_annual, SummaryAnnual);


// // 年度听歌报告2017-2023
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   const key =
//     ['2017', '2018', '2019'].indexOf(query.year) > -1 ? 'userdata' : 'data'
//   return request(
//     `/api/activity/summary/annual/${query.year}/${key}`,
//     data,
//     createOption(query),
//   )
// }