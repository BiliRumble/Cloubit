use crate::get_device_id;
use crate::models::http::RequestOption;
use crate::network::request::create_request;
use crate::{error::AppError, models::http::Response};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde_json::{json, Value};

pub async fn register_anonimous() -> Result<Response, AppError> {
    let option = RequestOption {
        crypto: Some("weapi".into()),
        ua: None,
        ip: None,
        real_ip: None,
        proxy: None,
        headers: None,
        e_r: None,
        cache: Some(false),
    };

    let device_id = get_device_id();
    let encoded_id = STANDARD.encode(format!(
        "{} {}",
        &device_id,
        cloudmusic_dll_encode_id(&device_id)
    ));
    let data: Value = json!({
        "username": encoded_id
    });

    create_request("/api/register/anonimous", data, option).await
}

fn cloudmusic_dll_encode_id(some_id: &&str) -> String {
    const ID_XOR_KEY: &[u8] = b"3go8&$8*3*3h0k(2)2";

    let xored_bytes: Vec<u8> = some_id
        .bytes()
        .enumerate()
        .map(|(i, b)| b ^ ID_XOR_KEY[i % ID_XOR_KEY.len()])
        .collect();

    let digest = md5::compute(&xored_bytes);
    STANDARD.encode(digest.as_ref())
}
