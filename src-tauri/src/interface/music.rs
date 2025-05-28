use crate::services::music::{search, get_song_url};
use serde_json::{Value};

#[tauri::command]
pub async fn api_search(
    keyword: String, 
    types: Option<u64>, 
    limit: Option<u64>, 
    offset: Option<u64>
) -> Result<Value, String> {
    // 设置默认值
    let search_type = types.unwrap_or(1);
    let search_limit = limit.unwrap_or(30);
    let search_offset = offset.unwrap_or(0);
    
    // 调用现有的 search 函数
    match search(keyword, search_type, search_limit, search_offset).await {
        Ok(response) => {
            // 返回响应体给前端
            Ok(response.body)
        }
        Err(error) => {
            Err(format!("搜索请求失败: {:?}", error))
        }
    }
}

#[tauri::command]
pub async fn api_get_song_url(
    id: u64,
    br: Option<u64>
) -> Result<Value, String> {
    let bitrate = br.unwrap_or(999000);
    
    match get_song_url(id, bitrate).await {
        Ok(response) => Ok(response.body),
        Err(error) => Err(format!("获取歌曲链接失败: {:?}", error))
    }
}