use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::{from_str, json, Value};
use std::str::FromStr;
use web::Query;

// 分类歌单
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/top/playlist").route(web::get().to(top_playlist)));
}

// 入参
define_request_struct!(TopPlaylist, {
    cat: Option<String>,
    order: Option<String>,
    limit: Option<u8>,
    offset: Option<u8>,
});

impl TopPlaylist {
    async fn requests(req: HttpRequest, query: Query<TopPlaylist>) -> Result<Response, Value> {
        let data = json!({
            "cat": query.cat.clone().unwrap_or("全部".to_string()),
            "order": query.order.clone().unwrap_or("hot".to_string()),
            "limit": query.limit.unwrap_or(50),
            "offset": query.offset.unwrap_or(0),
            "total": true,
        });
        let res = create_request(
            "/api/playlist/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await?;
        
        
        

        // 将返回结果转换为字符串并替换字段名
        let result = res.body.to_string().replace("avatarImgId_str", "avatarImgIdStr");

        // 将字符串转换回 JSON 对象
        let json_result: Value = from_str(result.as_str()).unwrap();

        Ok(Response {
            status: 200,
            body: json_result,
            cookie: None,
        })
    }
}
cache_handler!(top_playlist, TopPlaylist);



// // 分类歌单
// 
// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   const data = {
//     cat: query.cat || '全部', // 全部,华语,欧美,日语,韩语,粤语,小语种,流行,摇滚,民谣,电子,舞曲,说唱,轻音乐,爵士,乡村,R&B/Soul,古典,民族,英伦,金属,朋克,蓝调,雷鬼,世界音乐,拉丁,另类/独立,New Age,古风,后摇,Bossa Nova,清晨,夜晚,学习,工作,午休,下午茶,地铁,驾车,运动,旅行,散步,酒吧,怀旧,清新,浪漫,性感,伤感,治愈,放松,孤独,感动,兴奋,快乐,安静,思念,影视原声,ACG,儿童,校园,游戏,70后,80后,90后,网络歌曲,KTV,经典,翻唱,吉他,钢琴,器乐,榜单,00后
//     order: query.order || 'hot', // hot,new
//     limit: query.limit || 50,
//     offset: query.offset || 0,
//     total: true,
//   }
//   const res = await request(
//     `/api/playlist/list`,
//     data,
//     createOption(query, 'weapi'),
//   )
//   const result = JSON.stringify(res).replace(
//     /avatarImgId_str/g,
//     'avatarImgIdStr',
//   )
//   return JSON.parse(result)
// }