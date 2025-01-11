
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

// 用户贡献内容
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ugc/detail").route(web::get().to(ugc_detail)));
}

// 入参
define_request_struct!(UgcDetail, {
  auditStatus: Option<String>,
  limit: Option<u32>,
  offset: Option<u32>,
  order: Option<String>,
  sortBy: Option<String>,
  r#type: Option<u32>
});


impl UgcDetail {
    async fn requests(req: HttpRequest, query: Query<UgcDetail>) -> Result<Response, Value> {
        let data = json!({
          "auditStatus": query.auditStatus.clone().unwrap_or_default(),
          "limit": query.limit.unwrap_or(10),
          "offset": query.offset.unwrap_or(0),
          "order": query.order.clone().unwrap_or_else(|| "desc".to_string()),
          "sortBy": query.sortBy.clone().unwrap_or_else(|| "createTime".to_string()),
          "type": query.r#type.unwrap_or(1)
        });
        create_request(
            "/api/rep/ugc/detail",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(ugc_detail, UgcDetail);



// // 用户贡献内容
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     auditStatus: query.auditStatus || '',
//     //待审核:0 未采纳:-5 审核中:1 部分审核通过:4 审核通过:5
//     //WAIT:0 REJECT:-5 AUDITING:1 PARTLY_APPROVED:4 PASS:5
//     limit: query.limit || 10,
//     offset: query.offset || 0,
//     order: query.order || 'desc', //asc
//     sortBy: query.sortBy || 'createTime',
//     type: query.type || 1,
//     //曲库纠错 ARTIST:1 ALBUM:2 SONG:3 MV:4 LYRIC:5 TLYRIC:6
//     //曲库补充 ALBUM:101 MV:103
//   }
//   return request(`/api/rep/ugc/detail`, data, createOption(query, 'weapi'))
// }