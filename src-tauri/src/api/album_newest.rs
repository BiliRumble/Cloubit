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

// 最新专辑
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/album/newest").route(web::get().to(album_newest)));
}

// 入参
define_request_struct!(AlbumNewest, {});


impl AlbumNewest {
    async fn requests(req: HttpRequest, query: Query<AlbumNewest>) -> Result<Response, Value> {
        let data = json!({});
        create_request(
            "/api/discovery/newAlbum",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(album_newest, AlbumNewest);