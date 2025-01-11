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

// 云盘导入歌曲
pub fn configure(cfg: &mut web::ServiceConfig) {
    cfg.service(web::resource("/cloud/import").route(web::post().to(cloud_import)));
}

// 入参
define_request_struct!(CloudImport, {
    id: Option<i32>,
    artist: Option<String>,
    album: Option<String>,
    md5: String,
    bitrate: i32,
    fileSize: usize,
    song: String,
    fileType: String,
});

impl CloudImport {
    async fn requests(req: HttpRequest, query: Query<CloudImport>) -> Result<Response, Value> {
        let id = query.id.unwrap_or(-2);
        let artist = query.artist.clone().unwrap_or_else(|| "未知".to_string());
        let album = query.album.clone().unwrap_or_else(|| "未知".to_string());

        let check_data = json!({
            "uploadType": 0,
            "songs": json!([{
                "md5": query.md5,
                "songId": id,
                "bitrate": query.bitrate,
                "fileSize": query.fileSize,
            }]),
        });

        let res = create_request(
            "/api/cloud/upload/check/v2",
            check_data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await?;

        let import_data = json!({
            "uploadType": 0,
            "songs": json!([{
                "songId": res.body["data"][0]["songId"].as_i64().unwrap(),
                "bitrate": query.bitrate,
                "song": query.song,
                "artist": artist,
                "album": album,
                "fileName": format!("{}.{}", query.song, query.fileType),
            }]),
        });

        create_request(
            "/api/cloud/user/song/import",
            import_data,
            create_request_option(extract_headers!(req), &query.common, "weapi"),
        ).await
    }
}
cache_handler!(cloud_import, CloudImport);



// // 云盘导入歌曲
// const createOption = require('../util/option.js')
// module.exports = async (query, request) => {
//   query.id = query.id || -2
//   query.artist = query.artist || '未知'
//   query.album = query.album || '未知'
//   const checkData = {
//     uploadType: 0,
//     songs: JSON.stringify([
//       {
//         md5: query.md5,
//         songId: query.id,
//         bitrate: query.bitrate,
//         fileSize: query.fileSize,
//       },
//     ]),
//   }
//   const res = await request(
//     `/api/cloud/upload/check/v2`,
//     checkData,
//     createOption(query),
//   )
//   //res.body.data[0].upload 0:文件可导入,1:文件已在云盘,2:不能导入
//   //只能用song决定云盘文件名，且上传后的文件名后缀固定为mp3
//   const importData = {
//     uploadType: 0,
//     songs: JSON.stringify([
//       {
//         songId: res.body.data[0].songId,
//         bitrate: query.bitrate,
//         song: query.song,
//         artist: query.artist,
//         album: query.album,
//         fileName: query.song + '.' + query.fileType,
//       },
//     ]),
//   }
//   return request(`/api/cloud/user/song/import`, importData, createOption(query))
// }