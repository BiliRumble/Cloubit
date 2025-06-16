# Network

提供向网易云服务器发送请求的接口

## 调用示例

```rust
use crate::error::AppError;
use crate::network::request::{create_request, RequestOption, Response};
use serde_json::{json, Value};

pub async fn search() -> Result<Response, AppError> {
    // 构建请求参数
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

    // 构建请求数据
    let data = json!({
        "s": "Echo",
        "type": 1,
        "limit": 30,
        "offset": "0"
    });

    // 调用异步函数，并直接返回. create_request("API端口", 数据(可以为{}), 请求参数);
    create_request("/api/search/get", data, option).await // 返回包含body(Object)等
}
```
