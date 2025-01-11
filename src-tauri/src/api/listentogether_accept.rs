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
    cfg.service(web::resource("/listentogether/accept").route(web::get().to(listentogether_accept)));
}

// 入参
define_request_struct!(ListentogetherAccept, {
    room_id: String,
    inviter_id: String,
});

impl ListentogetherAccept {
    async fn requests(req: HttpRequest, query: Query<ListentogetherAccept>) -> Result<Response, Value> {
        let data = json!({
        "refer": "inbox_invite",
        "roomId": query.room_id,
        "inviterId": query.inviter_id,
    });
        create_request(
            "/api/listen/together/play/invitation/accept",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}
cache_handler!(listentogether_accept, ListentogetherAccept);


// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     refer: 'inbox_invite',
//     roomId: query.roomId,
//     inviterId: query.inviterId,
//   }
//   return request(
//     `/api/listen/together/play/invitation/accept`,
//     data,
//     createOption(query),
//   )
// }
