use crate::network::request::{create_request, RequestOption, Response};
use crate::network::text::cloudmusic_dll_encode_id;
use crate::DEVICE_ID;
use base64::engine::general_purpose::STANDARD;
use base64::Engine;
use serde_json::{json, Value};

pub async fn register_anonimous() -> Result<Response, Value> {
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

    let device_id = DEVICE_ID.clone();
    let encoded_id = STANDARD.encode(format!("{} {}", device_id, cloudmusic_dll_encode_id(&device_id)));
    let data: Value = json!({
        "username": encoded_id
    });

    // 调用异步函数，并直接返回
    create_request("/api/register/anonimous", data, option).await
}


// const CryptoJS = require('crypto-js')
// const path = require('path')
// const fs = require('fs')
// const ID_XOR_KEY_1 = '3go8&$8*3*3h0k(2)2'
// const deviceidText = fs.readFileSync(
//   path.resolve(__dirname, '../data/deviceid.txt'),
//   'utf-8',
// )
//
// const createOption = require('../util/option.js')
// const deviceidList = deviceidText.split('\n')
//
// function getRandomFromList(list) {
//   return list[Math.floor(Math.random() * list.length)]
// }
// function cloudmusic_dll_encode_id(some_id) {
//   let xoredString = ''
//   for (let i = 0; i < some_id.length; i++) {
//     const charCode =
//       some_id.charCodeAt(i) ^ ID_XOR_KEY_1.charCodeAt(i % ID_XOR_KEY_1.length)
//     xoredString += String.fromCharCode(charCode)
//   }
//   const wordArray = CryptoJS.enc.Utf8.parse(xoredString)
//   const digest = CryptoJS.MD5(wordArray)
//   return CryptoJS.enc.Base64.stringify(digest)
// }
//
// module.exports = async (query, request) => {
//   const deviceId = getRandomFromList(deviceidList)
//   global.deviceId = deviceId
//   const encodedId = CryptoJS.enc.Base64.stringify(
//     CryptoJS.enc.Utf8.parse(
//       `${deviceId} ${cloudmusic_dll_encode_id(deviceId)}`,
//     ),
//   )
//   const data = {
//     username: encodedId,
//   }
//   let result = await request(
//     `/api/register/anonimous`,
//     data,
//     createOption(query, 'weapi'),
//   )
//   if (result.body.code === 200) {
//     result = {
//       status: 200,
//       body: {
//         ...result.body,
//         cookie: result.cookie.join(';'),
//       },
//       cookie: result.cookie,
//     }
//   }
//   return result
// }
