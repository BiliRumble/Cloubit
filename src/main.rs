#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod crash_handler;
#[cfg(debug_assertions)]
use std::env;

fn main() {
    #[cfg(debug_assertions)]
    env::set_var("RUST_LOG", "debug");
    env_logger::init();

    let crash_handler = crash_handler::CrashHandler::new();
    crash_handler.install();
    crash_handler.cleanup_old_crashes(3);

    cloubit_lib::run();
}
