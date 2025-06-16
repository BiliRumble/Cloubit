use crate::error::AppError;
use crate::get_device_id;
use crate::network::request::{create_request, RequestOption, Response};
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde_json::{json, Value};

pub async fn register_anonimous() -> Result<Response, AppError> {
    let option = RequestOption {
        crypto: Some("weapi".to_string()),
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
        device_id,
        cloudmusic_dll_encode_id(&device_id)
    ));
    let data: Value = json!({
        "username": encoded_id
    });

    create_request("/api/register/anonimous", data, option).await
}

fn cloudmusic_dll_encode_id(some_id: &String) -> String {
    let id_xor_key_1 = "3go8&$8*3*3h0k(2)2";
    let some_id_bytes = some_id.as_bytes();
    let id_xor_key_1_bytes = id_xor_key_1.as_bytes();

    let xored_bytes: Vec<u8> = some_id_bytes
        .iter()
        .enumerate()
        .map(|(i, &b)| b ^ id_xor_key_1_bytes[i % id_xor_key_1_bytes.len()])
        .collect();

    let digest = md5::compute(&xored_bytes);
    STANDARD.encode(digest.as_ref())
}
