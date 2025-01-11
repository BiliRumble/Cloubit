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

// 歌单导入 - 元数据/文字/链接导入
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/import/name/task/create").route(web::post().to(playlist_import_name_task_create)));
}

// 入参
define_request_struct!(PlaylistImportNameTaskCreate, {
    importStarPlaylist: Option<bool>,
    local: Option<String>,
    playlistName: Option<String>,
    text: Option<String>,
    link: Option<String>,
});

impl PlaylistImportNameTaskCreate {
    async fn requests(req: HttpRequest, query: Query<PlaylistImportNameTaskCreate>) -> Result<Response, Value> {
        let mut data = json!({
            "importStarPlaylist": query.importStarPlaylist.unwrap_or(false),
        });

        if let Some(local) = &query.local {
            let local: Vec<Value> = serde_json::from_str(local).unwrap();
            let multi_songs = serde_json::to_string(
                &local.iter().map(|e| {
                    json!({
                        "songName": e["name"],
                        "artistName": e["artist"],
                        "albumName": e["album"],
                    })
                }).collect::<Vec<_>>()
            ).unwrap();
            data["multiSongs"] = json!(multi_songs);
        } else {
            let playlist_name = query.playlistName.clone().unwrap_or_else(|| format!("导入音乐 {}", chrono::Local::now().to_rfc3339()));
            let mut songs = String::new();
            if let Some(text) = &query.text {
                songs = serde_json::to_string(&vec![json!({
                    "name": playlist_name,
                    "type": "",
                    "url": format!("rpc://playlist/import?text={}", text),
                })]).unwrap();
            }
            if let Some(link) = &query.link {
                let link: Vec<String> = serde_json::from_str(link).unwrap();
                songs = serde_json::to_string(
                    &link.iter().map(|e| {
                        json!({
                            "name": playlist_name,
                            "type": "",
                            "url": e,
                        })
                    }).collect::<Vec<_>>()
                ).unwrap();
            }
            data["playlistName"] = json!(playlist_name);
            data["createBusinessCode"] = json!(null);
            data["extParam"] = json!(null);
            data["taskIdForLog"] = json!("");
            data["songs"] = json!(songs);
        }

        create_request(
            "/api/playlist/import/name/task/create",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}


cache_handler!(playlist_import_name_task_create, PlaylistImportNameTaskCreate);


// // 歌单导入 - 元数据/文字/链接导入
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   let data = {
//     importStarPlaylist: query.importStarPlaylist || false, // 导入我喜欢的音乐
//   }
// 
//   if (query.local) {
//     // 元数据导入
//     let local = JSON.parse(query.local)
//     let multiSongs = JSON.stringify(
//       local.map(function (e) {
//         return {
//           songName: e.name,
//           artistName: e.artist,
//           albumName: e.album,
//         }
//       }),
//     )
//     data = {
//       ...data,
//       multiSongs: multiSongs,
//     }
//   } else {
//     let playlistName = // 歌单名称
//       query.playlistName || '导入音乐 '.concat(new Date().toLocaleString())
//     let songs = ''
//     if (query.text) {
//       // 文字导入
//       songs = JSON.stringify([
//         {
//           name: playlistName,
//           type: '',
//           url: encodeURI('rpc://playlist/import?text='.concat(query.text)),
//         },
//       ])
//     }
// 
//     if (query.link) {
//       // 链接导入
//       let link = JSON.parse(query.link)
//       songs = JSON.stringify(
//         link.map(function (e) {
//           return { name: playlistName, type: '', url: encodeURI(e) }
//         }),
//       )
//     }
//     data = {
//       ...data,
//       playlistName: playlistName,
//       createBusinessCode: undefined,
//       extParam: undefined,
//       taskIdForLog: '',
//       songs: songs,
//     }
//   }
//   return request(
//     `/api/playlist/import/name/task/create`,
//     data,
//     createOption(query),
//   )
// }