use serde_json::Value;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AppError {
    Network(String),
    Crypto(String),
    InvalidHeader(String),
    Cookie(String),
    Cache(String),
    Json(String),
    Parse(String),
}

impl From<tauri_plugin_http::reqwest::header::InvalidHeaderValue> for AppError {
    fn from(err: tauri_plugin_http::reqwest::header::InvalidHeaderValue) -> Self {
        AppError::InvalidHeader(err.to_string())
    }
}

impl From<tauri_plugin_http::reqwest::Error> for AppError {
    fn from(err: tauri_plugin_http::reqwest::Error) -> Self {
        AppError::Network(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Json(err.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::Crypto(err.to_string())
    }
}

impl From<serde_json::Value> for AppError {
    fn from(value: serde_json::Value) -> Self {
        AppError::Json(value.to_string())
    }
}

impl From<sled::Error> for AppError {
    fn from(err: sled::Error) -> Self {
        AppError::Cache(err.to_string())
    }
}

impl<T> From<std::sync::PoisonError<T>> for AppError {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        AppError::Cache(format!("Mutex poisoned: {}", err))
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Network(msg) => write!(f, "Network error: {}", msg),
            AppError::Crypto(msg) => write!(f, "Crypto error: {}", msg),
            AppError::InvalidHeader(msg) => write!(f, "Invalid header: {}", msg),
            AppError::Cookie(msg) => write!(f, "Cookie error: {}", msg),
            AppError::Cache(msg) => write!(f, "Cache error: {}", msg),
            AppError::Json(msg) => write!(f, "JSON error: {}", msg),
            AppError::Parse(msg) => write!(f, "Parse error: {}", msg),
        }
    }
}

impl std::error::Error for AppError {}

impl From<AppError> for Value {
    fn from(err: AppError) -> Self {
        serde_json::json!({
            "error": err.to_string(),
            "code": 500
        })
    }
}
