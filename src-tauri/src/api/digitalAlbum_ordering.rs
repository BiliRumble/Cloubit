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

// // 购买数字专辑
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/digitalAlbum/ordering").route(web::get().to(digitalAlbum_ordering)));
}

// 入参
define_request_struct!(DigitalAlbumOrdering, {
    id: String,
    payment: String,
    quantity: String,
});

impl DigitalAlbumOrdering {
    async fn requests(req: HttpRequest, query: Query<DigitalAlbumOrdering>) -> Result<Response, Value> {
        let data = json!({
        "business": "Album",
        "paymentMethod": query.payment,
        "digitalResources": [
            {
            "business": "Album",
            "resourceID": query.id,
            "quantity": query.quantity,
            },
        ],
        "from": "web",
    });
        create_request(
            "/api/ordering/web/digital",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(digitalAlbum_ordering, DigitalAlbumOrdering);


// // 购买数字专辑
// 
// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     business: 'Album',
//     paymentMethod: query.payment,
//     digitalResources: JSON.stringify([
//       {
//         business: 'Album',
//         resourceID: query.id,
//         quantity: query.quantity,
//       },
//     ]),
//     from: 'web',
//   }
//   return request(
//     `/api/ordering/web/digital`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }
