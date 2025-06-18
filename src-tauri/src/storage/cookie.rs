use sled::Db;
use std::collections::HashMap;
use std::sync::{Arc, OnceLock, RwLock};
use tauri_plugin_http::reqwest::header::{HeaderMap, SET_COOKIE};

use crate::error::AppError;

#[derive(Clone)]
pub struct CookieManager {
    store: Arc<RwLock<HashMap<String, String>>>,
    db: Db,
}

static COOKIE_MANAGER: OnceLock<CookieManager> = OnceLock::new();

impl CookieManager {
    pub fn new(db: Db) -> Result<Self, AppError> {
        let store = match db.get("cookie_store") {
            Ok(Some(data)) => serde_json::from_slice(&data)?,
            Ok(None) => HashMap::new(),
            Err(e) => {
                return Err(AppError::Cache(format!(
                    "Failed to read cookie store: {}",
                    e
                )))
            }
        };

        Ok(Self {
            store: Arc::new(RwLock::new(store)),
            db,
        })
    }

    // 这里其实还有很大的优化空间，但是就目前来看够用
    pub fn update_from_headers(&self, headers: &HeaderMap) {
        if let Ok(mut store) = self.store.write() {
            for header in headers.get_all(SET_COOKIE) {
                if let Ok(cookie_str) = header.to_str() {
                    if let Some((key, value)) = parse_cookie(cookie_str) {
                        store.insert(key, value);
                    }
                }
            }

            if let Ok(json) = serde_json::to_vec(&*store) {
                let _ = self.db.insert("cookie_store", json);
            }
        }
    }

    pub fn get_header_value(&self) -> String {
        if let Ok(store) = self.store.read() {
            store
                .iter()
                .map(|(k, v)| format!("{}={}", k, v))
                .collect::<Vec<_>>()
                .join("; ")
        } else {
            String::new()
        }
    }

    #[allow(dead_code)]
    pub fn has_login_cookie(&self) -> bool {
        self.store
            .read()
            .map(|store| store.contains_key("MUSIC_U"))
            .unwrap_or(false)
    }

    #[allow(dead_code)]
    pub fn clear(&self) {
        if let Ok(mut store) = self.store.write() {
            store.clear();
            let _ = self.db.remove("cookie_store");
        }
    }
}

fn parse_cookie(header: &str) -> Option<(String, String)> {
    header.split(';').next().and_then(|kv| {
        let mut parts = kv.splitn(2, '=');
        match (parts.next(), parts.next()) {
            (Some(k), Some(v)) => Some((k.trim().to_string(), v.trim().to_string())),
            _ => None,
        }
    })
}

pub fn get_cookie_manager() -> &'static CookieManager {
    COOKIE_MANAGER.get_or_init(|| {
        CookieManager::new(crate::get_db().clone())
            .unwrap_or_else(|e| panic!("Failed to initialize cookie manager: {}", e))
    })
}
