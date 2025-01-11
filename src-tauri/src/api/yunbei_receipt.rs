
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


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/yunbei/receipt").route(web::get().to(yunbei_receipt)));
}

define_request_struct!(YunbeiReceipt, {
    limit: Option<u32>,
    offset: Option<u32>,
});

impl YunbeiReceipt {
    async fn requests(req: HttpRequest, query: Query<YunbeiReceipt>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.unwrap_or(10),
            "offset": query.offset.unwrap_or(0),
        });
        create_request(
            "/api/point/receipt",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(yunbei_receipt, YunbeiReceipt);



// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 10,
//     offset: query.offset || 0,
//   }
//   return request(`/api/point/receipt`, data, createOption(query))
// }