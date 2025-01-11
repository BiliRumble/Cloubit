use std::str::FromStr;
use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option, Response};
use crate::util::request::QueryOption;
use crate::{cache_handler, define_request_struct, extract_headers, success};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;
// 确保使用的是 reqwest::header::HeaderValue
// 路由配置
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/check/music").route(web::get().to(check_music)));
}

// 入参
define_request_struct!(CheckMusic, {
    id: String,
    br: Option<String>,
});


impl CheckMusic {
    async fn requests(req: HttpRequest, query: Query<CheckMusic>) -> Result<Response, Value> {
        let id = &query.id;
        let br = query.br.as_deref().unwrap_or("999000").to_string();

        let data = json!({
            "ids": format!("[{}]", id),
            "br": br
        });


        let response = create_request(
            "/api/song/enhance/player/url",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await;

        let mut playable = false;
        match response {
            Ok(mut response) => {
                let code = response.body.get("code").unwrap().as_i64().unwrap();
                if code == 200 {
                    let data = response.body.get("data").unwrap().as_array().unwrap();
                    if data.first().unwrap().get("code").unwrap().as_i64().unwrap() == 200 {
                        playable = true;
                    }
                }
                if playable {
                    response.body = success!(Some("ok".to_string()));
                    Ok(response)
                } else {
                    response.body = success!(Some("亲爱的,暂无版权".to_string()));
                    Ok(response)
                }
            }
            Err(e) => Err(e),
        }
    }
}

// 使用宏生成缓存处理函数
cache_handler!(check_music, CheckMusic);