use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use reqwest::Client;
use serde::Deserialize;
use serde_json::{json, Value};
use std::str::FromStr;
use web::Query;
// 相关歌单
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/related/playlist").route(web::get().to(related_playlist)));
}

// 入参
define_request_struct!(RelatedPlaylist, {
    id: String,
});

impl RelatedPlaylist {
    async fn requests(req: HttpRequest, query: Query<RelatedPlaylist>) -> Result<Response, Value> {
        let client = Client::new();
        let res = client
            .get(&format!("https://music.163.com/playlist?id={}", query.id))
            .send()
            .await;
        let body = res.unwrap().text().await.unwrap();
        let pattern = regex::Regex::new(r#"<div class="cver u-cover u-cover-3">[\s\S]*?<img src="([^"]+)">[\s\S]*?<a class="sname f-fs1 s-fc0" href="([^"]+)"[^>]*>([^<]+?)<\/a>[\s\S]*?<a class="nm nm f-thide s-fc3" href="([^"]+)"[^>]*>([^<]+?)<\/a>"#).unwrap();
        let mut playlists = vec![];
        for cap in pattern.captures_iter(&body) {
            playlists.push(json!({
                "creator": {
                    "userId": cap[4].replace("/user/home?id=", ""),
                    "nickname": &cap[5],
                },
                "coverImgUrl": cap[1].replace("?param=50y50", ""),
                "name": &cap[3],
                "id": cap[2].replace("/playlist?id=", ""),
            }));
        }
        Ok(Response {
            status: 200,
            body: json!({ "code": 200, "playlists": playlists }),
            cookie: None,
        })
    }
}

cache_handler!(related_playlist, RelatedPlaylist);

// // 相关歌单
// const { default: axios } = require('axios')
// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   const res = await axios({
//     method: 'GET',
//     url: `https://music.163.com/playlist?id=${query.id}`,
//   })
//   try {
//     const pattern =
//       /<div class="cver u-cover u-cover-3">[\s\S]*?<img src="([^"]+)">[\s\S]*?<a class="sname f-fs1 s-fc0" href="([^"]+)"[^>]*>([^<]+?)<\/a>[\s\S]*?<a class="nm nm f-thide s-fc3" href="([^"]+)"[^>]*>([^<]+?)<\/a>/g
//     let result,
//       playlists = []
//     while ((result = pattern.exec(res.data)) != null) {
//       playlists.push({
//         creator: {
//           userId: result[4].slice('/user/home?id='.length),
//           nickname: result[5],
//         },
//         coverImgUrl: result[1].slice(0, -'?param=50y50'.length),
//         name: result[3],
//         id: result[2].slice('/playlist?id='.length),
//       })
//     }
//     res.body = { code: 200, playlists: playlists }
//     return res
//   } catch (err) {
//     res.status = 500
//     res.body = { code: 500, msg: err.stack }
//     return Promise.reject(res)
//   }
// }
