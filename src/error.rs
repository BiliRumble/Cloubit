use serde_json::Value;
use std::fmt;

#[derive(Debug, Clone)]
pub enum AppError {
    Audio(String),
    Common(String),
    Crypto(String),
    Format(String),
    IO(String),
    Thread(String),
    Network(String),
}

impl From<reqwest::header::InvalidHeaderValue> for AppError {
    fn from(err: reqwest::header::InvalidHeaderValue) -> Self {
        AppError::Network(err.to_string())
    }
}

impl From<reqwest::Error> for AppError {
    fn from(err: reqwest::Error) -> Self {
        AppError::Network(err.to_string())
    }
}

impl From<serde_json::Error> for AppError {
    fn from(err: serde_json::Error) -> Self {
        AppError::Format(err.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::Common(err.to_string())
    }
}

impl From<serde_json::Value> for AppError {
    fn from(value: serde_json::Value) -> Self {
        AppError::Format(value.to_string())
    }
}

impl From<sled::Error> for AppError {
    fn from(err: sled::Error) -> Self {
        AppError::IO(err.to_string())
    }
}

impl<T> From<std::sync::PoisonError<T>> for AppError {
    fn from(err: std::sync::PoisonError<T>) -> Self {
        AppError::Thread(format!("Mutex poisoned: {}", err))
    }
}

impl From<rodio::decoder::DecoderError> for AppError {
    fn from(err: rodio::decoder::DecoderError) -> Self {
        AppError::Audio(err.to_string())
    }
}

impl From<rodio::source::SeekError> for AppError {
    fn from(err: rodio::source::SeekError) -> Self {
        AppError::Audio(err.to_string())
    }
}

impl From<std::sync::mpsc::RecvError> for AppError {
    fn from(err: std::sync::mpsc::RecvError) -> Self {
        AppError::Thread(err.to_string())
    }
}

impl fmt::Display for AppError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            AppError::Audio(msg) => write!(f, "Audio error: {}", msg),
            AppError::Common(msg) => write!(f, "Common error: {}", msg),
            AppError::Crypto(msg) => write!(f, "Crypto error: {}", msg),
            AppError::Format(msg) => write!(f, "Format error: {}", msg),
            AppError::IO(msg) => write!(f, "IO error: {}", msg),
            AppError::Thread(msg) => write!(f, "Thread error: {}", msg),
            AppError::Network(msg) => write!(f, "Network error: {}", msg),
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
