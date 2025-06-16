use sled::Db;
use std::env;
use std::sync::OnceLock;

static DB: OnceLock<Db> = OnceLock::new();

pub fn get_db() -> &'static Db {
    DB.get_or_init(|| {
        let db_path = env::var("APPDATA")
            .or_else(|_| env::var("HOME"))
            .unwrap_or_else(|_| ".".to_string());
        let db_path = format!("{}/.cloubit/database", db_path);
        if let Some(parent) = std::path::Path::new(&db_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        sled::open(db_path).expect("Failed to open database")
    })
}
