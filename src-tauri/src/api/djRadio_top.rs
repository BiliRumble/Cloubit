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

// //电台排行榜获取
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/djRadio/top").route(web::get().to(dj_radio_top)));
}

// 入参
define_request_struct!(DjRadioTop, {
    djRadioId: Option<String>,
    sortIndex: Option<i32>,
    dataGapDays: Option<i32>,
    dataType: Option<i32>,
});

impl DjRadioTop {
    async fn requests(req: HttpRequest, query: Query<DjRadioTop>) -> Result<Response, Value> {
        let data = json!({
            "djRadioId": query.djRadioId.clone().unwrap_or("".to_string()),
            "sortIndex": query.sortIndex.unwrap_or(1),
            "dataGapDays": query.dataGapDays.unwrap_or(7),
            "dataType": query.dataType.unwrap_or(3),
        });
        create_request(
            "/api/expert/worksdata/works/top/get",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(dj_radio_top, DjRadioTop);


// //电台排行榜获取
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     djRadioId: query.djRadioId || null, // 电台id
//     sortIndex: query.sortIndex || 1, // 排序 1:播放数 2:点赞数 3：评论数 4：分享数 5：收藏数
//     dataGapDays: query.dataGapDays || 7, // 天数 7:一周 30:一个月 90:三个月
//     dataType: query.dataType || 3, // 未知
//   }
//   return request(
//     '/api/expert/worksdata/works/top/get',
//     data,
//     createOption(query),
//   )
// }
