use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, to_string, Value};
use web::Query;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;

// 收藏单曲到歌单 从歌单删除歌曲
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/tracks").route(web::post().to(playlist_tracks)));
}

// 入参
define_request_struct!(PlaylistTracks, {
    op: String,
    pid: String,
    tracks: String,
});

impl PlaylistTracks {
    async fn requests(req: HttpRequest, query: Query<PlaylistTracks>) -> Result<Response, Value> {
        let tracks: Vec<&str> = query.tracks.split(',').collect();
        let data = json!({
            "op": query.op,
            "pid": query.pid,
            "trackIds": to_string(&tracks).unwrap(),
            "imme": "true",
        });

        let res = create_request(
            "/api/playlist/manipulate/tracks",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await;

        match res {
            Ok(response) => Ok(Response {
                status: 200,
                body: json!({
                    "code": 200,
                    "message": "操作成功",
                    "data": response.body,
                }),
                cookie: response.cookie,
            }),
            Err(error) => {
                if error.get("body").unwrap().get("code").unwrap().as_i64().unwrap_or(0) == 512 {
                    let mut all_tracks = tracks.clone();
                    all_tracks.extend(tracks.clone());
                    let data = json!({
                        "op": query.op,
                        "pid": query.pid,
                        "trackIds": to_string(&all_tracks).unwrap(),
                        "imme": "true",
                    });
                    create_request(
                        "/api/playlist/manipulate/tracks",
                        data,
                        create_request_option(extract_headers!(req), &query.common, "weapi"),
                    ).await
                } else {
                    Ok(Response {
                        status: 200,
                        body: error,
                        cookie: None,
                    })
                }
            }
        }
    }
}
cache_handler!(playlist_tracks, PlaylistTracks);



// // 收藏单曲到歌单 从歌单删除歌曲
// 
// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   //
//   const tracks = query.tracks.split(',')
//   const data = {
//     op: query.op, // del,add
//     pid: query.pid, // 歌单id
//     trackIds: JSON.stringify(tracks), // 歌曲id
//     imme: 'true',
//   }
// 
//   try {
//     const res = await request(
//       `/api/playlist/manipulate/tracks`,
//       data,
//       createOption(query, 'weapi'),
//     )
//     return {
//       status: 200,
//       body: {
//         ...res,
//       },
//     }
//   } catch (error) {
//     if (error.body.code === 512) {
//       return request(
//         `/api/playlist/manipulate/tracks`,
//         {
//           op: query.op, // del,add
//           pid: query.pid, // 歌单id
//           trackIds: JSON.stringify([...tracks, ...tracks]),
//           imme: 'true',
//         },
//         createOption(query, 'weapi'),
//       )
//     } else {
//       return {
//         status: 200,
//         body: error.body,
//       }
//     }
//   }
// }