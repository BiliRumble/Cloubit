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

// // 歌手动态信息
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/artist/detail/dynamic").route(web::get().to(artist_detail_dynamic)));
}

// 入参
define_request_struct!(ArtistDetailDynamic, {
    id: String,
});


impl ArtistDetailDynamic {
  async fn requests(req: HttpRequest, query: Query<ArtistDetailDynamic>) -> Result<Response, Value> {
    let data = json!({
            "id": query.id,
        });
    create_request(
      "/api/artist/detail/dynamic",
      data,
      create_request_option(extract_headers!(req), &query.common, ""),
    ).await
  }
}
cache_handler!(artist_detail_dynamic, ArtistDetailDynamic);



// // 歌手动态信息
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//   }
//   return request(`/api/artist/detail/dynamic`, data, createOption(query))
// }
