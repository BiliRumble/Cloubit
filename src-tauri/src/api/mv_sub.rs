
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

// 收藏与取消收藏MV
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/mv/sub").route(web::get().to(mv_sub)));
}

// 入参
define_request_struct!(MvSub, {
    mvid: String,
    t: i32,
});

impl MvSub {
    async fn requests(req: HttpRequest, query: Query<MvSub>) -> Result<Response, Value> {
        let t = if query.t == 1 { "sub" } else { "unsub" };
        let data = json!({
            "mvId": query.mvid,
            "mvIds": format!(r#"["{}"]"#, query.mvid),
        });
        create_request(
            &format!("/api/mv/{}", t),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(mv_sub, MvSub);


//// 收藏与取消收藏MV
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.t = query.t == 1 ? 'sub' : 'unsub'
//   const data = {
//     mvId: query.mvid,
//     mvIds: '["' + query.mvid + '"]',
//   }
//   return request(`/api/mv/${query.t}`, data, createOption(query, 'weapi'))
// }