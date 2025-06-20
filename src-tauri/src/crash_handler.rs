use backtrace::Backtrace;
use chrono::Utc;
use std::fs::File;
use std::io::Write;
use std::panic::{self};
use std::path::PathBuf;

pub struct CrashHandler {
    crash_dir: PathBuf,
}

impl CrashHandler {
    pub fn new() -> Self {
        let crash_dir = dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("cloubit")
            .join("crashes");
        std::fs::create_dir_all(&crash_dir).ok();
        Self { crash_dir }
    }

    pub fn install(&self) {
        let crash_dir = self.crash_dir.clone();

        panic::set_hook(Box::new(move |panic_info| {
            let timestamp = Utc::now();
            let filename = format!("crash_{}.txt", timestamp.format("%Y%m%d_%H%M%S_%f"));
            let funnies = vec![
                "That's we got, folks!",
                "Why you did do that?",
                "Well, this is awkward.",
                "I blame the developers.",
                "It appears we've hit a snag.",
                "The gremlins are at it again!",
                "I have no idea what I'm doing.",
                "This was not supposed to happen.",
                "This crash is brought to you by: Sleep Deprivation!",
            ];
            let seed = timestamp.timestamp_nanos_opt().unwrap_or_default() as usize;
            let funny_line = funnies[seed % funnies.len()];
            let crash_file_path = crash_dir.join(filename);

            if let Ok(mut file) = File::create(&crash_file_path) {
                writeln!(file, "=== CRASH REPORT ===").ok();
                writeln!(file, "// {}", funny_line).ok();
                writeln!(
                    file,
                    "Timestamp: {}",
                    timestamp.format("%Y-%m-%d %H:%M:%S UTC")
                )
                .ok();
                writeln!(file, "Version: {}", env!("CARGO_PKG_VERSION")).ok();
                writeln!(file, "").ok();

                writeln!(file, "Panic Information:").ok();
                if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                    writeln!(file, "  Message: {}", s).ok();
                } else if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                    writeln!(file, "  Message: {}", s).ok();
                } else {
                    writeln!(file, "  Message: Unknown panic payload").ok();
                }
                if let Some(location) = panic_info.location() {
                    writeln!(
                        file,
                        "  Location: {}:{}:{}",
                        location.file(),
                        location.line(),
                        location.column()
                    )
                    .ok();
                }
                writeln!(file, "").ok();

                writeln!(file, "Stack Trace:").ok();
                let bt = Backtrace::new();
                writeln!(file, "{:?}", bt).ok();
                writeln!(file, "").ok();

                writeln!(file, "System Information:").ok();
                writeln!(
                    file,
                    "  OS: {},{}",
                    std::env::consts::OS,
                    std::env::consts::FAMILY
                )
                .ok();
                writeln!(file, "  Architecture: {}", std::env::consts::ARCH).ok();

                log::error!(
                    "A crash has occurred. A crash dump has been saved to {:?}",
                    crash_file_path
                );
                std::process::exit(1);
            } else {
                log::error!("Failed to create crash dump file");
            }
        }));
    }

    pub fn cleanup_old_crashes(&self, max_files: usize) {
        if let Ok(entries) = std::fs::read_dir(&self.crash_dir) {
            let mut crash_files: Vec<_> = entries
                .filter_map(|entry| entry.ok())
                .filter(|entry| {
                    entry.path().extension().map_or(false, |ext| ext == "txt")
                        && entry.file_name().to_string_lossy().starts_with("crash_")
                })
                .collect();

            crash_files.sort_by_key(|entry| {
                entry
                    .metadata()
                    .and_then(|m| m.modified())
                    .unwrap_or(std::time::SystemTime::UNIX_EPOCH)
            });

            if crash_files.len() > max_files {
                for old_file in crash_files.iter().take(crash_files.len() - max_files) {
                    std::fs::remove_file(old_file.path()).ok();
                }
            }
        }
    }
}
