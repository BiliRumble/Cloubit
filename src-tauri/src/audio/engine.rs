use crate::audio::backend::AudioBackend;
use log::error;
use std::sync::OnceLock;

static BACKEND: OnceLock<AudioBackend> = OnceLock::new();

pub fn get_backend() -> &'static AudioBackend {
    BACKEND.get_or_init(|| match AudioBackend::new() {
        Ok(backend) => backend,
        Err(e) => {
            error!("Failed to initialize audio backend: {}", e);
            AudioBackend::dummy() // 考虑panic
        }
    })
}
