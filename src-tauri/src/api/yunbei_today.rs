
use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;


// 配置服务
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/yunbei/today").route(web::get().to(yunbei_today)));
}

// 入参
define_request_struct!(YunbeiToday, {});

impl YunbeiToday {
    async fn requests(req: HttpRequest, query: Query<YunbeiToday>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/point/today/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(yunbei_today, YunbeiToday);



// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(`/api/point/today/get`, data, createOption(query, 'weapi'))
// }