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

// // 首页-发现 dragon ball
// // 这个接口为移动端接口，首页-发现页（每日推荐、歌单、排行榜 那些入口）
// // 数据结构可以参考 https://github.com/hcanyz/flutter-netease-music-api/blob/master/lib/src/api/uncategorized/bean.dart#L290 HomeDragonBallWrap
// // !需要登录或者游客登录，非登录返回 []
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/homepage/dragon/ball").route(web::get().to(homepage_dragon_ball)));
}

// 入参
define_request_struct!(HomepageDragonBall, {
});

impl HomepageDragonBall {
    async fn requests(req: HttpRequest, query: Query<HomepageDragonBall>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/homepage/dragon/ball/static",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(homepage_dragon_ball, HomepageDragonBall);


// // 首页-发现 dragon ball
// // 这个接口为移动端接口，首页-发现页（每日推荐、歌单、排行榜 那些入口）
// // 数据结构可以参考 https://github.com/hcanyz/flutter-netease-music-api/blob/master/lib/src/api/uncategorized/bean.dart#L290 HomeDragonBallWrap
// // !需要登录或者游客登录，非登录返回 []
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
// 
//   return request(`/api/homepage/dragon/ball/static`, data, createOption(query))
// }
