use crate::network::text::cookie_string_to_json;
use serde_json::{Value, Map};
use std::sync::{Arc, Mutex};
use lazy_static::lazy_static;

// 认证管理器结构
pub struct AuthManager {
    user_cookie: Arc<Mutex<Option<String>>>,
    music_u_token: Arc<Mutex<Option<String>>>,
    music_a_token: Arc<Mutex<Option<String>>>,
    is_logged_in: Arc<Mutex<bool>>,
}

impl AuthManager {
    pub fn new() -> Self {
        let mut manager = Self {
            user_cookie: Arc::new(Mutex::new(None)),
            music_u_token: Arc::new(Mutex::new(None)),
            music_a_token: Arc::new(Mutex::new(None)),
            is_logged_in: Arc::new(Mutex::new(false)),
        };
        
        // 从数据库加载已保存的认证信息
        manager.load_from_database();
        manager
    }

    // 从数据库加载认证信息
    fn load_from_database(&mut self) {
        // 加载用户cookie
        if let Ok(Some(cookie_data)) = crate::db.get("user_cookie") {
            if let Ok(cookie_str) = String::from_utf8(cookie_data.to_vec()) {
                self.set_user_cookie(&cookie_str);
            }
        }

        // 加载匿名token
        if let Ok(Some(token_data)) = crate::db.get("anonymousToken") {
            if let Ok(token_str) = String::from_utf8(token_data.to_vec()) {
                if let Ok(mut music_a) = self.music_a_token.lock() {
                    *music_a = Some(token_str);
                }
            }
        }
    }

    // 设置用户cookie
    pub fn set_user_cookie(&self, cookie: &str) {
        // 解析cookie
        let cookie_obj = cookie_string_to_json(cookie);
        
        // 更新用户cookie
        if let Ok(mut user_cookie) = self.user_cookie.lock() {
            *user_cookie = Some(cookie.to_string());
        }

        // 提取MUSIC_U token
        if let Some(music_u) = cookie_obj.get("MUSIC_U").and_then(|v| v.as_str()) {
            if let Ok(mut token) = self.music_u_token.lock() {
                *token = Some(music_u.to_string());
            }
            
            // 保存到数据库
            let _ = crate::db.insert("music_u_token", music_u);
        }

        // 更新登录状态
        if let Ok(mut logged_in) = self.is_logged_in.lock() {
            *logged_in = cookie_obj.contains_key("MUSIC_U");
        }

        // 保存到数据库
        let _ = crate::db.insert("user_cookie", cookie);
        let _ = crate::db.flush();

        println!("用户认证信息已更新");
    }

    // 获取当前用户cookie作为JSON对象
    pub fn get_user_cookie_as_json(&self) -> Option<Value> {
        if let Ok(cookie_guard) = self.user_cookie.lock() {
            if let Some(cookie_str) = cookie_guard.as_ref() {
                let cookie_obj = cookie_string_to_json(cookie_str);
                return Some(Value::Object(cookie_obj));
            }
        }
        None
    }

    // 获取原始cookie字符串
    pub fn get_user_cookie_string(&self) -> Option<String> {
        if let Ok(cookie_guard) = self.user_cookie.lock() {
            cookie_guard.clone()
        } else {
            None
        }
    }

    // 检查是否已登录
    pub fn is_logged_in(&self) -> bool {
        if let Ok(logged_in) = self.is_logged_in.lock() {
            *logged_in
        } else {
            false
        }
    }

    // 获取MUSIC_U token
    pub fn get_music_u_token(&self) -> Option<String> {
        if let Ok(token_guard) = self.music_u_token.lock() {
            token_guard.clone()
        } else {
            None
        }
    }

    // 设置匿名token
    pub fn set_anonymous_token(&self, token: &str) {
        if let Ok(mut music_a) = self.music_a_token.lock() {
            *music_a = Some(token.to_string());
        }
        let _ = crate::db.insert("anonymousToken", token);
        let _ = crate::db.flush();
    }

    // 获取匿名token
    pub fn get_anonymous_token(&self) -> Option<String> {
        if let Ok(token_guard) = self.music_a_token.lock() {
            token_guard.clone()
        } else {
            None
        }
    }

    // 合并认证cookie到请求cookie中
    pub fn merge_auth_cookie(&self, base_cookie: &Value) -> Value {
        let mut merged_cookie = base_cookie.as_object().cloned().unwrap_or_else(|| Map::new());

        // 合并用户cookie
        if let Some(user_cookie) = self.get_user_cookie_as_json() {
            if let Some(user_cookie_obj) = user_cookie.as_object() {
                for (key, value) in user_cookie_obj {
                    merged_cookie.insert(key.clone(), value.clone());
                }
            }
        }

        // 如果没有MUSIC_U，使用匿名token
        if !merged_cookie.contains_key("MUSIC_U") {
            if let Some(anonymous_token) = self.get_anonymous_token() {
                merged_cookie.insert("MUSIC_A".to_string(), Value::String(anonymous_token));
            }
        }

        Value::Object(merged_cookie)
    }

    // 清除认证信息（登出）
    pub fn logout(&self) {
        if let Ok(mut user_cookie) = self.user_cookie.lock() {
            *user_cookie = None;
        }
        if let Ok(mut music_u) = self.music_u_token.lock() {
            *music_u = None;
        }
        if let Ok(mut logged_in) = self.is_logged_in.lock() {
            *logged_in = false;
        }

        // 从数据库删除
        let _ = crate::db.remove("user_cookie");
        let _ = crate::db.remove("music_u_token");
        let _ = crate::db.flush();

        println!("用户已登出");
    }

    // 获取认证状态摘要
    pub fn get_auth_status(&self) -> Value {
        serde_json::json!({
            "isLoggedIn": self.is_logged_in(),
            "hasMusicU": self.get_music_u_token().is_some(),
            "hasAnonymousToken": self.get_anonymous_token().is_some(),
            "cookieLength": self.get_user_cookie_string().map(|s| s.len()).unwrap_or(0)
        })
    }
}

// 全局认证管理器实例
lazy_static! {
    pub static ref AUTH_MANAGER: AuthManager = AuthManager::new();
}

// 便捷函数
pub fn get_auth_manager() -> &'static AuthManager {
    &AUTH_MANAGER
}

// 便捷的全局函数
pub fn set_user_cookie(cookie: &str) {
    AUTH_MANAGER.set_user_cookie(cookie);
}

pub fn get_current_user_cookie() -> Option<Value> {
    AUTH_MANAGER.get_user_cookie_as_json()
}

pub fn is_user_logged_in() -> bool {
    AUTH_MANAGER.is_logged_in()
}

pub fn logout_user() {
    AUTH_MANAGER.logout();
}

pub fn get_auth_status() -> Value {
    AUTH_MANAGER.get_auth_status()
}
