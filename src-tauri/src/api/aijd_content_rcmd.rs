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
use std::time::{SystemTime, UNIX_EPOCH};
use web::Query;


// 私人 DJ

// 实际请求参数如下, 部分内容省略, 敏感信息已进行混淆
// 可按需修改此 API 的代码
/* {"extInfo":"{\"lastRequestTimestamp\":1692358373509,\"lbsInfoList\":[{\"lat\":40.23076381,\"lon\":129.07545186,\"time\":1692358543},{\"lat\":40.23076381,\"lon\":129.07545186,\"time\":1692055283}],\"listenedTs\":false,\"noAidjToAidj\":true}","header":"{}"} */
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/aidj/content/rcmd").route(web::get().to(aidj_content_rcmd)));
}

// 入参
define_request_struct!(AidjContentRcmd, {
    latitude: Option<String>,
    longitude: Option<String>,
});


impl AidjContentRcmd {
    async fn requests(req: HttpRequest, query: Query<AidjContentRcmd>) -> Result<Response, Value> {
        // 初始化 extInfo 为一个 JSON 对象
        let mut ext_info = json!({
            "noAidjToAidj": false,
            "lastRequestTimestamp": SystemTime::now()
                .duration_since(UNIX_EPOCH)
                .unwrap()
                .as_millis(),
            "listenedTs": false,
        });

        // 如果存在 latitude 和 longitude，添加 lbsInfoList 信息
        if let (Some(latitude_str), Some(longitude_str)) = (&query.latitude, &query.longitude) {
            // 尝试将 latitude 和 longitude 从 String 转换为 f64
            if let (Ok(latitude), Ok(longitude)) = (
                latitude_str.parse::<f64>(),
                longitude_str.parse::<f64>(),
            ) {
                ext_info["lbsInfoList"] = json!([
                    {
                        "lat": latitude,
                        "lon": longitude,
                        "time": SystemTime::now()
                            .duration_since(UNIX_EPOCH)
                            .unwrap()
                            .as_secs(),
                    }
                ]);
            }
        }

        // 将 extInfo 直接放入 data 中，而不序列化为字符串
        let data = json!({
            "extInfo": ext_info,
        });

        create_request(
            "/api/aidj/content/rcmd/info",
            data,
            create_request_option(extract_headers!(req), &query.common, ""),
        ).await
    }
}





// 使用宏生成缓存处理函数
cache_handler!(aidj_content_rcmd, AidjContentRcmd);