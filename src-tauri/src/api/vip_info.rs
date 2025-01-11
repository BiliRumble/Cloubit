

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


// 获取 VIP 信息
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/vip/info").route(web::get().to(vip_info)));
}

// 入参
define_request_struct!(VipInfo, {
    uid: Option<String>
});

impl VipInfo {
    async fn requests(req: HttpRequest, query: Query<VipInfo>) -> Result<Response, Value> {
        let data = json!({
            "userId": query.uid.clone().unwrap_or_default()
        });
        create_request(
            "/api/music-vip-membership/front/vip/info",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(vip_info, VipInfo);


// // 获取 VIP 信息
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(
//     `/api/music-vip-membership/front/vip/info`,
//     {
//       userId: query.uid || '',
//     },
//     createOption(query, 'weapi'),
//   )
// }