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

// 数字专辑-新碟上架
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/album/list").route(web::get().to(album_list)));
}

// 入参
define_request_struct!(AlbumList, {
    offset: Option<String>,
    area: Option<String>,
    limit: Option<String>,
    r#type: Option<String>,
});


impl AlbumList {
    async fn requests(req: HttpRequest, query: Query<AlbumList>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.clone().unwrap_or("30".to_string()),
            "offset": query.offset.clone().unwrap_or("0".to_string()),
            "total": true,
            "area": query.area.clone().unwrap_or("ALL".to_string()),  //ALL:全部,ZH:华语,EA:欧美,KR:韩国,JP:日本
            "type": query.r#type.clone(),
        });
        create_request(
            "/api/vipmall/albumproduct/list",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(album_list, AlbumList);