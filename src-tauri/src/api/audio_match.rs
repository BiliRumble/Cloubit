use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::ClientBuilder;
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use std::time::Duration;
use web::Query;

// // 歌手单曲
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/audio/match").route(web::get().to(audio_match)));
}

// 入参
define_request_struct!(AudioMatch, {
    duration: i32,
    audioFP: String,
});


impl AudioMatch {
    async fn requests(req: HttpRequest, query: Query<AudioMatch>) -> Result<Response, Value> {
        let client_builder = ClientBuilder::new()
            .timeout(Duration::from_secs(10)) // 请求超时
            .connect_timeout(Duration::from_secs(5)) // 连接超时
            .pool_max_idle_per_host(30)
            .pool_idle_timeout(Duration::from_secs(90)) // 设置连接池的空闲超时时间
            .gzip(true); // 启用 Gzip 压缩
        let client = client_builder.build().map_err(|e| format!("Client build error: {}", e))?;
        let query = query.into_inner();
        let url = format!("https://interface.music.163.com/api/music/audio/match?sessionId=0123456789abcdef&algorithmCode=shazam_v2&duration={}&rawdata={}&times=1&decrypt=1", query.duration, query.audioFP);
        let result = client.get(&url).send().await.map_err(|e| format!("Request error: {}", e));
        match result {
            Ok(response) => {
                let body = response.text().await.map_err(|e| format!("Response body error: {}", e))?;
                Ok(Response {
                    status: 200,
                    body: json!({"code": 200, "data": body}),
                    cookie: None,
                })
            }
            Err(e) => Err(json!({"code": 500, "message": e})),
        }
    }
}
cache_handler!(audio_match, AudioMatch);


// const { default: axios } = require('axios')
// 
// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   const res = await axios({
//     method: 'get',
//     url: `https://interface.music.163.com/api/music/audio/match?sessionId=0123456789abcdef&algorithmCode=shazam_v2&duration=${
//       query.duration
//     }&rawdata=${encodeURIComponent(query.audioFP)}&times=1&decrypt=1`,
//     data: null,
//   })
//   return {
//     status: 200,
//     body: {
//       code: 200,
//       data: res.data.data,
//     },
//   }
// }
