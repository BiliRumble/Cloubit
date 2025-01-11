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


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/get/userids").route(web::get().to(get_userids)));
}

// 入参
define_request_struct!(GetUserids, {
    nicknames: String,
});

impl GetUserids {
    async fn requests(req: HttpRequest, query: Query<GetUserids>) -> Result<Response, Value> {
        let data = json!({
        "nicknames": query.nicknames.clone(),
    });
        create_request(
            "/api/user/getUserIds",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(get_userids, GetUserids);


// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     nicknames: query.nicknames,
//   }
//   return request(`/api/user/getUserIds`, data, createOption(query, 'weapi'))
// }
