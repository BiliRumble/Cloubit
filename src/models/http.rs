use serde_json::Value;
use tauri::http::HeaderMap;

#[derive(Default, Debug)]
pub struct RequestOption {
    pub crypto: Option<String>,
    pub ip: Option<String>,
    pub real_ip: Option<String>,
    pub headers: Option<HeaderMap>,
    pub proxy: Option<String>,
    pub e_r: Option<bool>,
    pub ua: Option<String>,
    pub cache: Option<bool>,
}

#[derive(Debug, Clone)]
pub struct Response {
    #[allow(dead_code)]
    pub status: u16,
    pub body: Value,
    pub cookie: Option<Vec<String>>,
    #[allow(dead_code)]
    pub headers: HeaderMap,
}
