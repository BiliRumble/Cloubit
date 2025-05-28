use crate::services::auth::{check_qr, create_qr, get_qr_unikey, login_status, logout};
use serde_json::{Value, json};
use tauri::{AppHandle, Emitter, Listener};
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};
use tokio::time::sleep;

#[tauri::command]
pub async fn api_login(app_handle: AppHandle) -> Result<Value, String> {
    let session_id = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_millis();
    
    // 取消之前的会话
    let _ = app_handle.emit("cancel-login", &json!({"timestamp": session_id}));
    
    // 获取数据
    let unikey = get_qr_unikey().await
        .map_err(|e| format!("获取unikey失败: {:?}", e))?
        .body.get("unikey").and_then(|v| v.as_str())
        .ok_or("无法获取unikey")?.to_string();
    
    let qr_data = create_qr(unikey.clone(), true).await
        .map_err(|e| format!("生成二维码失败: {}", e))?;
    
    // 启动轮询
    tokio::spawn(poll_qr_status(app_handle.clone(), session_id, unikey.clone()));

    Ok(json!({
        "code": 200,
        "data": {"unikey": unikey, "qrUrl": qr_data, "sessionId": session_id}
    }))
}

async fn poll_qr_status(app_handle: AppHandle, session_id: u128, unikey: String) {
    let cancelled = Arc::new(AtomicBool::new(false));
    let cancelled_clone = cancelled.clone();
    
    // 监听取消事件
    let _listener = app_handle.listen("cancel-login", move |event| {
        if let Ok(payload) = serde_json::from_str::<Value>(event.payload()) {
            if let Some(ts) = payload.get("timestamp").and_then(|v| v.as_u64()) {
                if ts as u128 > session_id {
                    cancelled_clone.store(true, Ordering::Relaxed);
                }
            }
        }
    });
    
    emit_status(&app_handle, session_id, 801, "等待扫码", "waiting").await;
    
    for poll_count in 0..60 {
        if cancelled.load(Ordering::Relaxed) { break; }
        if poll_count >= 60 {
            emit_status(&app_handle, session_id, 800, "二维码已过期", "expired").await;
            break;
        }
        
        match check_qr(unikey.clone()).await {
            Ok(resp) => {
                match resp.body.get("code").and_then(|v| v.as_i64()).unwrap_or(0) {
                    800 => { emit_status(&app_handle, session_id, 800, "二维码已过期", "expired").await; break; }
                    801 => emit_status(&app_handle, session_id, 801, "等待扫码", "waiting").await,
                    802 => emit_status(&app_handle, session_id, 802, "已扫码，等待确认", "scanned").await,
                    803 => {
                        emit_status(&app_handle, session_id, 803, "登录成功", "success").await;
                        let cookie: String = resp.cookie.unwrap_or_else(|| vec![]).join("; ");
                        let cookie_obj = crate::network::text::cookie_string_to_json(&cookie);
                        println!("{:?}", cookie_obj.clone());
                        if let Some(music_u) = cookie_obj.get("MUSIC_U").and_then(|v| v.as_str()) {
                            let _ = crate::db.insert("music_u_token", music_u);
                            let _ = crate::db.flush();
                            println!("用户登录成功，MUSIC_U已保存: {}", &music_u[..20]); // 只显示前20个字符
                        }
                        break;
                    }                    
                    code => emit_status(&app_handle, session_id, code, "未知状态", "unknown").await,
                }
            }
            Err(_) => {
                emit_status(&app_handle, session_id, 500, "检查失败", "error").await;
                sleep(Duration::from_secs(2)).await;
            }
        }
        
        sleep(Duration::from_secs(3)).await;
    }
}

async fn emit_status(app_handle: &AppHandle, session_id: u128, code: i64, message: &str, status: &str) {
    let _ = app_handle.emit("qr-status", &json!({
        "code": code, "message": message, "status": status, "sessionId": session_id
    }));
}

#[tauri::command]
pub async fn api_login_status() -> Result<Value, String> {
    match login_status().await {
        Ok(response) => Ok(response.body),
        Err(error) => Err(format!("获取登录信息失败: {:?}", error))
    }
}

#[tauri::command]
pub async fn api_logout() -> Result<Value, String> {
    //  清除cookie
    let _ = crate::db.remove("music_u_token");
    match logout().await {
        Ok(response) => Ok(response.body),
        Err(error) => Err(format!("登出失败: {:?}", error))
    }
}