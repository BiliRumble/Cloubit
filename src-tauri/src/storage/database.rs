use sled::Db;
use std::sync::OnceLock;

static DB: OnceLock<Db> = OnceLock::new();

pub fn get_db() -> &'static Db {
    DB.get_or_init(|| {
        let db_path = dirs::data_local_dir()
            .or_else(|| dirs::data_dir())
            .or_else(|| dirs::home_dir())
            .unwrap_or_else(|| std::env::current_dir().unwrap_or_else(|_| ".".into()))
            .join("cloubit")
            .join("database");

        if let Some(parent) = db_path.parent() {
            if let Err(e) = std::fs::create_dir_all(parent) {
                log::warn!("Failed to create database directory: {}", e);
            }
        }

        let config = sled::Config::default()
            .path(&db_path)
            .cache_capacity(64 * 1024 * 1024)
            .flush_every_ms(Some(1000))
            .compression_factor(4);

        config.open().unwrap_or_else(|e| {
            panic!("Could not open database: {}", e);
        })
    })
}
