
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


// // 通过传过来的歌单id拿到所有歌曲数据
// // 支持传递参数limit来限制获取歌曲的数据数量 例如: /playlist/track/all?id=7044354223&limit=10
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/track/all").route(web::post().to(playlist_track_all)));
}


define_request_struct!(PlaylistTrackAll, {
    id: String,
    s: Option<u32>,
    limit: Option<u32>,
    offset: Option<u32>,
});

impl PlaylistTrackAll {
    async fn requests(req: HttpRequest, query: Query<PlaylistTrackAll>) -> Result<Response, Value> {
        let data = json!({
            "id": query.id,
            "n": 100000,
            "s": query.s.unwrap_or(8),
        });

        let limit = query.limit.unwrap_or(u32::MAX);
        let offset = query.offset.unwrap_or(0);

        let res = create_request(
            "/api/v6/playlist/detail",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await?;

        let track_ids: Vec<Value> = res.body["playlist"]["trackIds"]
            .as_array()
            .unwrap()
            .iter()
            .skip(offset as usize)
            .take(limit as usize)
            .map(|item| json!({"id": item["id"]}))
            .collect();

        let ids_data = json!({
            "c": serde_json::to_string(&track_ids).unwrap(),
        });

        create_request(
            "/api/v3/song/detail",
            ids_data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}


cache_handler!(playlist_track_all, PlaylistTrackAll);



// // 通过传过来的歌单id拿到所有歌曲数据
// // 支持传递参数limit来限制获取歌曲的数据数量 例如: /playlist/track/all?id=7044354223&limit=10
//
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     id: query.id,
//     n: 100000,
//     s: query.s || 8,
//   }
//   //不放在data里面避免请求带上无用的数据
//   let limit = parseInt(query.limit) || Infinity
//   let offset = parseInt(query.offset) || 0
//
//   return request(`/api/v6/playlist/detail`, data, createOption(query)).then(
//     (res) => {
//       let trackIds = res.body.playlist.trackIds
//       let idsData = {
//         c:
//           '[' +
//           trackIds
//             .slice(offset, offset + limit)
//             .map((item) => '{"id":' + item.id + '}')
//             .join(',') +
//           ']',
//       }
//
//       return request(
//         `/api/v3/song/detail`,
//         idsData,
//         createOption(query, 'weapi'),
//       )
//     },
//   )
// }