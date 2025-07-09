#[derive(Debug, Clone)]
pub enum BackendState {
    Set(String),
    #[allow(dead_code)]
    Play(bool),
    #[allow(dead_code)]
    Speed(f32),
    #[allow(dead_code)]
    Volume(f32),
    #[allow(dead_code)]
    Seek(u64),
}
