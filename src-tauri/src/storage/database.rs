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

        sled::open(&db_path).unwrap_or_else(|e| {
            log::error!("Failed to open database at {:?}: {}", db_path, e);
            let fallback_path = std::path::Path::new(".").join("cloubit_db");
            sled::open(fallback_path).expect("Failed to open fallback database")
        })
    })
}
