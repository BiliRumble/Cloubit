
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

// 会员成长值领取记录
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/vip/growthpoint/details").route(web::get().to(vip_growthpoint_details)));
}

// 入参
define_request_struct!(VipGrowthPointDetails, {
  limit: Option<u32>,
  offset: Option<u32>
});

impl VipGrowthPointDetails {
    async fn requests(req: HttpRequest, query: Query<VipGrowthPointDetails>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.unwrap_or(20),
            "offset": query.offset.unwrap_or(0),
        });
        create_request(
            "/api/vipnewcenter/app/level/growth/details",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(vip_growthpoint_details, VipGrowthPointDetails);



// // 会员成长值领取记录
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     limit: query.limit || 20,
//     offset: query.offset || 0,
//   }
//   return request(
//     `/api/vipnewcenter/app/level/growth/details`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }