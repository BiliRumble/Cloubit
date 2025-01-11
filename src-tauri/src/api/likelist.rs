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

// // 喜欢的歌曲(无序)
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/likelist").route(web::get().to(likelist)));
}

// 入参
define_request_struct!(LikeList, {
    uid: String,
});

impl LikeList {
    async fn requests(req: HttpRequest, query: Query<LikeList>) -> Result<Response, Value> {
        let data = json!({
        "uid": query.uid, 
    });
        create_request(
            "/api/song/likelist/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(likelist, LikeList);


// // 喜欢的歌曲(无序)
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     uid: query.uid,
//   }
//   return request(`/api/song/likelist/get`, data, createOption(query, 'weapi'))
// }
