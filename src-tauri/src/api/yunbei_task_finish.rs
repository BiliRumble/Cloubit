use crate::util::cache::{get_cached_data, set_cached_data, AppState};
use crate::util::request::{create_request, create_request_option};
use crate::util::request::{QueryOption, Response};
use crate::{cache_handler, define_request_struct, extract_headers};
use actix_web::http::StatusCode;
use actix_web::{web, HttpRequest, HttpResponse, Responder};
use serde::Deserialize;
use serde_json::{json, Value};
use web::Query;
use reqwest::header::{HeaderMap, HeaderName, HeaderValue};
use std::str::FromStr;


pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/yunbei/task/finish").route(web::get().to(yunbei_task_finish)));
}

define_request_struct!(YunbeiTaskFinish, {
    user_task_id: String,
    deposit_code: Option<String>,
});

impl YunbeiTaskFinish {
    async fn requests(req: HttpRequest, query: Query<YunbeiTaskFinish>) -> Result<Response, Value> {
        let data = json!({
            "userTaskId": query.user_task_id,
            "depositCode": query.deposit_code.clone().unwrap_or_else(|| "0".to_string()),
        });
        create_request(
            "/api/usertool/task/point/receive",
            data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(yunbei_task_finish, YunbeiTaskFinish);


// const createOption = require('../util/option.js')
// module.exports = (query, request) => {
//   const data = {
//     userTaskId: query.userTaskId,
//     depositCode: query.depositCode || '0',
//   }
//   return request(
//     `/api/usertool/task/point/receive`,
//     data,
//     createOption(query, 'weapi'),
//   )
// }