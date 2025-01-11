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

// // 一起听 更新播放列表
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/listentogether/sync/list/command").route(web::get().to(listentogether_sync_list_command)));
}

// 入参
define_request_struct!(ListentogetherSyncListCommand, {
    roomId: String,
    commandType: String,
    userId: String,
    version: String,
    randomList: String,
    displayList: String,

});

impl ListentogetherSyncListCommand {
    async fn requests(req: HttpRequest, query: Query<ListentogetherSyncListCommand>) -> Result<Response, Value> {
        let data = json!({
        "roomId": query.roomId,
        "playlistParam": {
            "commandType": query.commandType,
            "version": [
            {
                "userId": query.userId,
                "version": query.version,
            },
            ],
            "anchorSongId": "",
            "anchorPosition": -1,
            "randomList": query.randomList.split(',').collect::<Vec<&str>>(),
            "displayList": query.displayList.split(',').collect::<Vec<&str>>(),
        },
    });
        create_request(
            "/api/listen/together/sync/list/command/report",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(listentogether_sync_list_command, ListentogetherSyncListCommand);

// // 一起听 更新播放列表
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     roomId: query.roomId,
//     playlistParam: JSON.stringify({
//       commandType: query.commandType,
//       version: [
//         {
//           userId: query.userId,
//           version: query.version,
//         },
//       ],
//       anchorSongId: '',
//       anchorPosition: -1,
//       randomList: query.randomList.split(','),
//       displayList: query.displayList.split(','),
//     }),
//   }
//   return request(
//     `/api/listen/together/sync/list/command/report`,
//     data,
//     createOption(query),
//   )
// }
