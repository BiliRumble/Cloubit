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

// 数字专辑-语种风格馆
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/album/list/style").route(web::get().to(album_list_style)));
}

// 入参
define_request_struct!(AlbumListStyle, {
    offset: Option<String>,
    area: Option<String>,
    limit: Option<String>,
});


impl AlbumListStyle {
    async fn requests(req: HttpRequest, query: Query<AlbumListStyle>) -> Result<Response, Value> {
        let data = json!({
            "limit": query.limit.clone().unwrap_or("10".to_string()),
            "offset": query.offset.clone().unwrap_or("0".to_string()),
            "total": true,
            "area": query.area.clone().unwrap_or("Z_H".to_string()),  //Z_H:华语,E_A:欧美,KR:韩国,JP:日本
        });
        create_request(
            "/api/vipmall/appalbum/album/style",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(album_list_style, AlbumListStyle);