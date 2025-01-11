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

// // 收藏与取消收藏歌手
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/artist/sub").route(web::get().to(artist_sub)));
}

// 入参
define_request_struct!(ArtistSub, {
    id: String,
    t: Option<i8>,
});


impl ArtistSub {
    async fn requests(req: HttpRequest, query: Query<ArtistSub>) -> Result<Response, Value> {
        let t = query.t.unwrap_or(1);
        let t = if t == 1 { "sub" } else { "unsub" };
        // 创建请求数据
        let data = json!({
            "artistId": query.id,
            "artistIds": format!("[{}]", query.id),
        });

        create_request(
            &format!("/api/artist/{}/", t),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(artist_sub, ArtistSub);


// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.t = query.t == 1 ? 'sub' : 'unsub'
//   const data = {
//     artistId: query.id,
//     artistIds: '[' + query.id + ']',
//   }
//   return request(`/api/artist/${query.t}`, data, createOption(query, 'weapi'))
// }
