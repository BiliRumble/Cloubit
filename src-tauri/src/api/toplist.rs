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

// 所有榜单介绍
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/toplist").route(web::get().to(toplist)));
}


// 入参
define_request_struct!(Toplist, {
    
});


impl Toplist {
    async fn requests(req: HttpRequest, query: Query<Toplist>) -> Result<Response, Value> {
        create_request(
            "/api/toplist",
            json!({}),
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}


cache_handler!(toplist, Toplist);


// // 所有榜单介绍
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(`/api/toplist`, {}, createOption(query))
// }