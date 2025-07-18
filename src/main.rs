mod audio;
mod core;
mod crash_handler;
mod error;
mod models;
mod network;
mod service;
mod storage;

use error::AppError;

use crate::core::app;

fn main() -> Result<(), AppError> {
    env_logger::init();

    let crash_handler = crash_handler::CrashHandler::new();
    crash_handler.install();
    crash_handler.cleanup_old_crashes(3);

    app::run()?;
    let _ = slint::run_event_loop_until_quit();

    Ok(())
}
