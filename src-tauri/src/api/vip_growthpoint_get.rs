
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

// 领取会员成长值
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/vip/growthpoint/get").route(web::get().to(vip_growthpoint_get)));
}

// 入参
define_request_struct!(VipGrowthPointGet, {
  ids: Vec<String>
});

impl VipGrowthPointGet {
    async fn requests(req: HttpRequest, query: Query<VipGrowthPointGet>) -> Result<Response, Value> {
        let data = json!({
            "taskIds": query.ids,
        });
        create_request(
            "/api/vipnewcenter/app/level/task/reward/get",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(vip_growthpoint_get, VipGrowthPointGet);



// // 领取会员成长值
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     taskIds: query.ids,
//   }
//   return request(
//     `/api/vipnewcenter/app/level/task/reward/get`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }