use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

// 云盘导入歌曲
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/cloud/import").route(web::post().to(cloud_import)));
}

// 入参
define_request_struct!(Chorus, {
    ids: Vec<i64>,
});

impl Chorus {
    async fn requests(req: HttpRequest, query: Query<Chorus>) -> Result<Response, Value> {
		let data = json!({
			"ids": [query.ids],
		});

        create_request(
            &format!("/api/v1/album/{}", query.id),
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
	}
}
cache_handler!(cloud_import, CloudImport);



// // 副歌时间
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(
//     `/api/song/chorus`,
//     {
//       ids: JSON.stringify([query.id]),
//     },
//     createOption(query),
//   )
// }
