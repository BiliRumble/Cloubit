
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

// 云盘数据详情
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/cloud/detail").route(web::get().to(user_cloud_detail)));
}

// 入参
define_request_struct!(UserCloudDetail, {
    id: String,
});

impl UserCloudDetail {
    async fn requests(req: HttpRequest, query: Query<UserCloudDetail>) -> Result<Response, Value> {
        // 将 id 字符串分割成 Vec<&str>
        let id_string = query.id.replace(" ", "");
        let id: Vec<&str> = id_string.split(',').collect();

        let data = json!({
            "songIds": id,
        });

        let request_option = create_request_option(extract_headers!(req), &query.common, "weapi");

        create_request(
            "/api/v1/cloud/get/byids",
            data,
            request_option,
        ).await
    }
}
cache_handler!(user_cloud_detail, UserCloudDetail);


// // 云盘数据详情
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const id = query.id.replace(/\s/g, '').split(',')
//   const data = {
//     songIds: id,
//   }
//   return request(`/api/v1/cloud/get/byids`, data, createOption(query, 'weapi'))
// }