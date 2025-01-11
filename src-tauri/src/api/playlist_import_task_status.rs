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


// 歌单导入 - 任务状态
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/import/task/status").route(web::get().to(playlist_import_task_status)));
}

// 入参
define_request_struct!(PlaylistImportTaskStatus, {
    id: String,
});

impl PlaylistImportTaskStatus {
    async fn requests(req: HttpRequest, query: Query<PlaylistImportTaskStatus>) -> Result<Response, Value> {
        let data = json!({
            "taskIds": serde_json::to_string(&vec![query.id.clone()]).unwrap(),
        });
        create_request(
            "/api/playlist/import/task/status/v2",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}


cache_handler!(playlist_import_task_status, PlaylistImportTaskStatus);


// // 歌单导入 - 任务状态
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   return request(
//     `/api/playlist/import/task/status/v2`,
//     {
//       taskIds: JSON.stringify([query.id]),
//     },
//     createOption(query),
//   )
// }