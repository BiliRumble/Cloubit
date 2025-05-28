use crate::network::request::{create_request, RequestOption, Response};
use serde_json::{json, Value};

pub async fn search(keyword: String, type_: u64, limit: u64, offset: u64) -> Result<Response, Value> {
    let option = RequestOption {
        crypto: None,
        cookie: None,
        ua: None,
        ip: None,
        real_ip: None,
        proxy: None,
        headers: None,
        e_r: None,
    };

    // type 2000要特殊处理
    if type_ == 2000 {
        let data: Value = json!({
            "keyword": keyword,
            "scene": "normal",
            "limit": limit,
            "offset": offset
        });

        create_request("/api/search/voice/get", data, option).await
    } else {
        let data = json!({
            "s": keyword,
            // 1: 单曲, 10: 专辑, 100: 歌手, 1000: 歌单, 1002: 用户, 1004: MV, 1006: 歌词, 1009: 电台, 1014: 视频
            "type": type_,
            "limit": limit,
            "offset": offset
        });
    
        // 调用异步函数，并直接返回
        create_request("/api/search/get", data, option).await
    }
}

pub async fn get_song_url(
    song_id: u64,
    br: u64
) -> Result<Response, Value> {
    let option = RequestOption {
        crypto: None,
        cookie: None,
        ua: None,
        ip: None,
        real_ip: None,
        proxy: None,
        headers: None,
        e_r: None,
    };

    let data = json!({
        "ids": format!("[{}]", song_id),
        "br": br,
    });

    create_request("/api/song/enhance/player/url", data, option).await
}