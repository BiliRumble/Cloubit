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

// // 首页-发现 block page
// // 这个接口为移动端接口，首页-发现页，数据结构可以参考 https://github.com/hcanyz/flutter-netease-music-api/blob/master/lib/src/api/uncategorized/bean.dart#L259 HomeBlockPageWrap
// // query.refresh 是否刷新数据
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/homepage/block/page").route(web::get().to(homepage_block_page)));
}

// 入参
define_request_struct!(HomepageBlockPage, {
    refresh: Option<bool>,
    cursor: String,
});

impl HomepageBlockPage {
    async fn requests(req: HttpRequest, query: Query<HomepageBlockPage>) -> Result<Response, Value> {
        let data = json!({
            "refresh": query.refresh.unwrap_or(false),
            "cursor": query.cursor,
    });
        create_request(
            "/api/homepage/block/page",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(homepage_block_page, HomepageBlockPage);


// // 首页-发现 block page
// // 这个接口为移动端接口，首页-发现页，数据结构可以参考 https://github.com/hcanyz/flutter-netease-music-api/blob/master/lib/src/api/uncategorized/bean.dart#L259 HomeBlockPageWrap
// // query.refresh 是否刷新数据
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = { refresh: query.refresh || false, cursor: query.cursor }
//   return request(`/api/homepage/block/page`, data, createOption(query, 'weapi'))
// }
