use std::{
    hash::{DefaultHasher, Hash, Hasher},
    sync::{Arc, Mutex, OnceLock},
};

use cached::TimedCache;
use serde_json::Value;

use crate::models::http::Response;

static REQUEST_CACHE: OnceLock<Arc<Mutex<TimedCache<String, Response>>>> = OnceLock::new();

pub fn get_cache() -> &'static Arc<Mutex<TimedCache<String, Response>>> {
    REQUEST_CACHE.get_or_init(|| {
        Arc::new(Mutex::new(TimedCache::with_lifespan_and_capacity(
            300, 1000,
        )))
    })
}

pub fn generate_cache_key(uri: &str, data: &Value) -> String {
    let mut hasher = DefaultHasher::new();
    uri.hash(&mut hasher);
    if let Ok(json_str) = serde_json::to_string(data) {
        json_str.hash(&mut hasher);
    }
    format!("{:x}", hasher.finish())
}
