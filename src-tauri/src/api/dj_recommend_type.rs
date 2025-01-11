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

// // 精选电台分类
// 
// /*
//     有声书 10001
//     知识技能 453050
//     商业财经 453051
//     人文历史 11
//     外语世界 13
//     亲子宝贝 14
//     创作|翻唱 2001
//     音乐故事 2
//     3D|电子 10002
//     相声曲艺 8
//     情感调频 3
//     美文读物 6
//     脱口秀 5
//     广播剧 7
//     二次元 3001
//     明星做主播 1
//     娱乐|影视 4
//     科技科学 453052
//     校园|教育 4001
//     旅途|城市 12
// */
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/dj/recommend/type").route(web::get().to(dj_recommend_type)));
}

// 入参
define_request_struct!(DjRecommendType, {
    r#type: i32,
});

impl DjRecommendType {
    async fn requests(req: HttpRequest, query: Query<DjRecommendType>) -> Result<Response, Value> {
        let data = json!({
            "cateId": query.r#type,
        });
        create_request(
            "/api/djradio/recommend",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(dj_recommend_type, DjRecommendType);


// // 精选电台分类
// 
// /*
//     有声书 10001
//     知识技能 453050
//     商业财经 453051
//     人文历史 11
//     外语世界 13
//     亲子宝贝 14
//     创作|翻唱 2001
//     音乐故事 2
//     3D|电子 10002
//     相声曲艺 8
//     情感调频 3
//     美文读物 6
//     脱口秀 5
//     广播剧 7
//     二次元 3001
//     明星做主播 1
//     娱乐|影视 4
//     科技科学 453052
//     校园|教育 4001
//     旅途|城市 12
// */
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     cateId: query.type,
//   }
//   return request(`/api/djradio/recommend`, data, createOption(query, 'weapi'))
// }
