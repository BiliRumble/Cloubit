#[macro_export]
macro_rules! error {
    // 错误响应，包含 code 和 message
    ($code:expr, $message:expr) => {{
        serde_json::json!({
            "code": $code.unwrap_or(500),
            "success": false,
            "message": $message.unwrap_or_else(|| "请求失败".to_string())
        })
    }};
    
    // 只传 message 时，默认 code 为 500
    ($message:expr) => {{
        error!(Some(500), Some($message))
    }};
    
    // 默认错误响应，code 和 message 都为 None
    () => {{
        error!(Some(500), None)
    }};
}

#[macro_export]
macro_rules! success {
    // 成功响应，包含 message 和 data
    ($message:expr, $data:expr) => {{
        let mut value = serde_json::json!({
            "code": 200,
            "success": true,
            "message": $message.unwrap_or_else(|| "请求成功".to_string()),
        });
        if let Some(data) = $data {
            value.as_object_mut().unwrap().insert("data".to_string(), data);
        }
        value
    }};
    
    // 只传 message 时，默认 data 为 None
    ($message:expr) => {{
        success!($message, None)
    }};
    
    // 只传 data 时，默认 message 为 None
    ($data:expr) => {{
        success!(None, Some($data))
    }};
    
    // 默认成功响应，message 和 data 都为 None
    () => {{
        success!(None, None)
    }};
}