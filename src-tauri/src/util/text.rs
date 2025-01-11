use base64::{engine::general_purpose::STANDARD, Engine};
use crypto::digest::Digest;
use crypto::md5::Md5;
use rand::{Rng, RngCore};
use serde_json::{Map, Value};

pub fn cookie_obj_to_string(cookie: Map<String, Value>) -> String {
    cookie.iter()
        .map(|(key, value)| format!("{}={}", key, value))
        .collect::<Vec<String>>()
        .join("; ")
}


pub fn cookie_string_to_json(cookie: &str) -> Map<String, Value> {
    let cookie_arr = cookie.split(';');
    let mut obj = Map::new();
    for i in cookie_arr {
        let arr = i.split('=').collect::<Vec<&str>>();
        if arr.len() == 2 {
            obj.insert(arr[0].trim().to_string(), Value::String(arr[1].trim().to_string()));
        }
    }
    obj
}


pub fn generate_random_hex_string(size: usize) -> String {
    // 创建一个大小为 `size` 的字节数组
    let mut buffer = vec![0u8; size];

    // 用随机字节填充数组
    rand::thread_rng().fill_bytes(&mut buffer);

    // 转换为十六进制字符串
    hex::encode(buffer)
}


pub fn json_to_urlencoded(data: &Value) -> String {
    let mut serializer = form_urlencoded::Serializer::new(String::new());

    if let Some(map) = data.as_object() {
        for (key, value) in map {
            match value {
                Value::String(s) => {
                    serializer.append_pair(key, s);
                }
                Value::Number(num) => {
                    serializer.append_pair(key, &num.to_string());
                }
                Value::Bool(b) => {
                    serializer.append_pair(key, &b.to_string());
                }
                _ => {
                    // Skip unsupported types (e.g., nested objects, arrays, null)
                }
            }
        }
    }

    serializer.finish()
}

// 辅助函数：将 Map<String, Value> 转换为 Cookie 格式字符串
pub fn map_to_cookie_header(header: &Map<String, Value>) -> String {
    header.iter()
        .map(|(key, value)| {
            let encoded_key = form_urlencoded::byte_serialize(key.as_bytes()).collect::<String>();
            let encoded_value = form_urlencoded::byte_serialize(value.as_str().unwrap_or_default().as_bytes()).collect::<String>();
            format!("{}={}", encoded_key, encoded_value)
        })
        .collect::<Vec<String>>()
        .join("; ")
}


pub fn choose_user_agent(crypto: &str, ua_type: &str) -> String {
    let user_agent_map = [
        ("weapi", "pc", "Mozilla/5.0 (Macintosh; Intel Mac OS X 10_15_7) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/124.0.0.0 Safari/537.36 Edg/124.0.0.0"),
        ("linuxapi", "linux", "Mozilla/5.0 (X11; Linux x86_64) AppleWebKit/537.36 (KHTML, like Gecko) Chrome/60.0.3112.90 Safari/537.36"),
        ("api", "pc", "Mozilla/5.0 (Windows NT 10.0; WOW64) AppleWebKit/537.36 (KHTML, like Gecko) Safari/537.36 Chrome/91.0.4472.164 NeteaseMusicDesktop/3.0.18.203152"),
        ("api", "android", "NeteaseMusic/9.1.65.240927161425(9001065);Dalvik/2.1.0 (Linux; U; Android 14; 23013RK75C Build/UKQ1.230804.001)"),
        ("api", "iphone", "NeteaseMusic 9.0.90/5038 (iPhone; iOS 16.2; zh_CN)"),
    ];
    match user_agent_map.iter().find(|(key, value, _)| key == &crypto && value == &ua_type) {
        Some(x) => x,
        None => &{
            println!("{} {}", crypto, ua_type);
            ("", "", "")
        }
    }.2.to_string()
}


pub fn wnmcid() -> String {
    let characters = "abcdefghijklmnopqruvwxyz";
    let mut random_string = String::new();
    for _ in 0..6 {
        let index = rand::thread_rng().gen_range(0..characters.len());
        random_string.push(characters.chars().nth(index).unwrap());
    }
    format!("{}.01.0", random_string)
}



pub fn cloudmusic_dll_encode_id(some_id: &String) -> String {
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


#[test]
fn test1(){
    let encode_id=cloudmusic_dll_encode_id(&"4D773E9694127C96EB4BF707DF1CC5B5EEC64F1E33C4EE9F4C26".to_string());
    assert_eq!(encode_id,"Vw9Z8XGaGpKYRcDhONKxbg==")
}


//   toBoolean(val) {
//     if (typeof val === 'boolean') return val
//     if (val === '') return val
//     return val === 'true' || val == '1'
//   },

pub fn to_boolean(val: &str) -> bool {
    if val == "true" || val == "1" {
        return true;
    }
    false
}