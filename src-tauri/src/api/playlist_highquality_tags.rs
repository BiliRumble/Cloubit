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

// 精品歌单 tags
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/highquality/tags").route(web::get().to(playlist_highquality_tags)));
}

// 入参
define_request_struct!(PlaylistHighQualityTags, {

});


impl PlaylistHighQualityTags {
    async fn requests(req: HttpRequest, query: Query<PlaylistHighQualityTags>) -> Result<Response, Value> {
        create_request(
            "/api/playlist/highquality/tags",
            json!({}),
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}


cache_handler!(playlist_highquality_tags, PlaylistHighQualityTags);


// // 精品歌单 tags
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {}
//   return request(
//     `/api/playlist/highquality/tags`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }