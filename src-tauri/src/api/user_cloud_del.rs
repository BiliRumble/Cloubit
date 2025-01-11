
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

// 云盘歌曲删除
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/user/cloud/del").route(web::get().to(user_cloud_del)));
}

// 入参
define_request_struct!(UserCloudDel, {
    id: String,
});

impl UserCloudDel {
    async fn requests(req: HttpRequest, query: Query<UserCloudDel>) -> Result<Response, Value> {
        let data = json!({
            "songIds": [query.id],
        });
        create_request(
            "/api/cloud/del",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(user_cloud_del, UserCloudDel);



// // 云盘歌曲删除
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     songIds: [query.id],
//   }
//   return request(`/api/cloud/del`, data, createOption(query, 'weapi'))
// }