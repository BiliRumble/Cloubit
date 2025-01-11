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

// 首页轮播图
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/banner").route(web::get().to(banner)));
}

// 入参
define_request_struct!(Banner, {
    r#type: Option<i32>,
});


impl Banner {
    async fn requests(req: HttpRequest, query: Query<Banner>) -> Result<Response, Value> {
        const TYPE: [&str; 4] = ["pc", "android", "iphone", "ipad"];
        let r#type = query.r#type.unwrap_or(0);
        let data = json!({
            "clientType": TYPE.get(r#type as usize).unwrap_or(&"pc"),
        });
        create_request(
            "/api/v2/banner/get",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(banner, Banner);


// 首页轮播图
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//  const type =
//  {
//      0: 'pc',
//      1: 'android',
//      2: 'iphone',
//      3: 'ipad',
//  }[query.type || 0] || 'pc'
//  return request(
//      `/api/v2/banner/get`,
//      { clientType: type },
//      createOption(query),
//  )
// }
