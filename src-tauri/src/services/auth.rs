use crate::network::request::{create_request, RequestOption, Response};
use base64::{prelude::BASE64_STANDARD, Engine};
use qrcode::{render::svg, QrCode};
use serde_json::{json, Value};

pub async fn get_qr_unikey() -> Result<Response, Value> {
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
        "type": 3
    });

    create_request("/api/login/qrcode/unikey", data, option).await
}

pub async fn create_qr(key: String, qrimg: bool) -> Result<String, String> {
    let url = format!("https://music.163.com/login?codekey={}", key);

    if qrimg {
        match QrCode::new(&url) {
            Ok(code) => {
                let svg_string = code.render::<svg::Color>()
                    .min_dimensions(200, 200)
                    .dark_color(svg::Color("#000000"))
                    .light_color(svg::Color("#ffffff"))
                    .build();
                
                // 直接返回base64编码的SVG
                let base64_image = BASE64_STANDARD.encode(svg_string.as_bytes());
                let data_uri = format!("data:image/svg+xml;base64,{}", base64_image);
                Ok(data_uri)
            }
            Err(e) => Err(format!("生成二维码失败: {}", e)),
        }
    } else {
        // 如果不需要图片，直接返回URL
        Ok(url)
    }
}

pub async fn check_qr(key: String) -> Result<Response, Value> {
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
        "key": key,
        "type": 3,
    });

    create_request("/api/login/qrcode/client/login", data, option).await
}

pub async fn login_status() -> Result<Response, Value> {
    let option = RequestOption {
        crypto: Some("weapi".to_string()),
        cookie: None,
        ua: None,
        ip: None,
        real_ip: None,
        proxy: None,
        headers: None,
        e_r: None,
    };

    create_request("/api/w/nuser/account/get", json!({}), option).await
}

pub async fn logout() -> Result<Response, Value> {
    let option = RequestOption {
        crypto: Some("weapi".to_string()),
        cookie: None,
        ua: None,
        ip: None,
        real_ip: None,
        proxy: None,
        headers: None,
        e_r: None,
    };

    create_request("/api/logout", json!({}), option).await
}