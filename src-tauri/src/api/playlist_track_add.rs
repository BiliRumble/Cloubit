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


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/playlist/track/add").route(web::post().to(playlist_track_add)));
}



define_request_struct!(PlaylistTrackAdd, {
    pid: String,
    ids: Option<String>,
});

impl PlaylistTrackAdd {
    async fn requests(req: HttpRequest, query: Query<PlaylistTrackAdd>) -> Result<Response, Value> {
        let ids = query.ids.clone().unwrap_or_default();
        let tracks: Vec<Value> = ids.split(',')
            .map(|item| json!({"type": 3, "id": item}))
            .collect();
        let data = json!({
            "id": query.pid,
            "tracks": serde_json::to_string(&tracks).unwrap(),
        });

        create_request(
            "/api/playlist/track/add",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}

cache_handler!(playlist_track_add, PlaylistTrackAdd);


// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   query.ids = query.ids || ''
//   const data = {
//     id: query.pid,
//     tracks: JSON.stringify(
//       query.ids.split(',').map((item) => {
//         return { type: 3, id: item }
//       }),
//     ),
//   }
//   console.log(data)
//
//   return request(`/api/playlist/track/add`, data, createOption(query, 'weapi'))
// }