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

// // 数字专辑销量
pub fn configure(cfg: &mut web::ServiceConfig) {
  cfg.service(web::resource("/digitalAlbum/sales").route(web::get().to(digitalAlbum_sales)));
}

// 入参
define_request_struct!(DigitalAlbumSeles, {
    ids: String,
});

impl DigitalAlbumSeles {
  async fn requests(req: HttpRequest, query: Query<DigitalAlbumSeles>) -> Result<Response, Value> {
    let data = json!({
        "albumIds": query.ids,
    });
    create_request(
      "/api/vipmall/albumproduct/album/query/sales",
      data,
      create_request_option(extract_headers!(req), &query.common, "weapi"),
    ).await
  }
}
cache_handler!(digitalAlbum_sales, DigitalAlbumSeles);


// // 数字专辑销量
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     albumIds: query.ids,
//   }
//   return request(
//     `/api/vipmall/albumproduct/album/query/sales`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
