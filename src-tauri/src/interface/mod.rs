pub mod music;
pub mod auth;

pub use music::{
    api_search,
    api_get_song_url
};
pub use auth::{
    api_login,
    api_login_status,
    api_logout
};