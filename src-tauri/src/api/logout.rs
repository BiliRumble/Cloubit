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


// 退出登录
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/logout").route(web::get().to(logout)));
}

// 退出登录
define_request_struct!(Logout, {});

impl Logout {
    async fn requests(req: HttpRequest, query: Query<Logout>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/logout",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}



cache_handler!(logout, Logout);


// 退出登录

/*const createOption = require('../util/option.js')
                             module.exports = (query, request) => {
                               return request(`/api/logout`, {}, createOption(query))
                             }*/
