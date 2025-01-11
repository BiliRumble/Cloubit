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

// // 每日推荐歌曲-不感兴趣
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/recommend/songs/dislike").route(web::get().to(recommend_songs_dislike)));
}

// 入参
define_request_struct!(RecommendSongsDislike, {
    id: String,
});

impl RecommendSongsDislike {
    async fn requests(req: HttpRequest, query: Query<RecommendSongsDislike>) -> Result<Response, Value> {
        let data = json!({
            "resId": query.id,
            "resType": 4,
            "sceneType": 1,
        });
        create_request(
            "/api/v2/discovery/recommend/dislike",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(recommend_songs_dislike, RecommendSongsDislike);


//