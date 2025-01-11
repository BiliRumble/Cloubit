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

// // 收藏/取消收藏专辑
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/album/sub").route(web::get().to(album_sub)));
}

// 入参
define_request_struct!(AlbumSub, {
    id:String,
    t:i8,
});


impl AlbumSub {
    async fn requests(req: HttpRequest, query: Query<AlbumSub>) -> Result<Response, Value> {
        let t = if query.t == 1 { "sub" } else { "unsub" };
        let data = json!({
          "id":query.id
        });
        create_request(
            &format!("/api/album/{}", t)
            ,
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(album_sub, AlbumSub);


// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   query.t = query.t == 1 ? 'sub' : 'unsub'
//   const data = {
//     id: query.id,
//   }
//   return request(`/api/album/${query.t}`, data, createOption(query, 'weapi'))
// }

