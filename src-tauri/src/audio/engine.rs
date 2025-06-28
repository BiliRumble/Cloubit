use crate::audio::backend::AudioBackend;
use std::sync::OnceLock;

static BACKEND: OnceLock<AudioBackend> = OnceLock::new();

pub fn get_backend() -> &'static AudioBackend {
    BACKEND.get_or_init(|| AudioBackend::new())
}
