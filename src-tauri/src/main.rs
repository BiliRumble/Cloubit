// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
#![allow(non_snake_case, unused_extern_crates, dead_code, unused_variables)]

#[tokio::main]
async fn main() {
    cloubit_lib::run();
}

