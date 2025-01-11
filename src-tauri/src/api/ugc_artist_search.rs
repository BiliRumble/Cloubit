
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

// 搜索歌手
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/ugc/artist/search").route(web::get().to(ugc_artist_search)));
}

// 入参
define_request_struct!(UgcArtistSearch, {
  keyword: Option<String>,
  limit: Option<u32>
});


impl UgcArtistSearch {
    async fn requests(req: HttpRequest, query: Query<UgcArtistSearch>) -> Result<Response, Value> {
        let data = json!({
          "keyword": query.keyword.clone().unwrap_or_default(),
          "limit": query.limit.unwrap_or(40)
        });
        create_request(
            "/api/rep/ugc/artist/search",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(ugc_artist_search, UgcArtistSearch);


// // 搜索歌手
// // 可传关键字或者歌手id
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     keyword: query.keyword,
//     limit: query.limit || 40,
//   }
//   return request(`/api/rep/ugc/artist/search`, data, createOption(query))
// }