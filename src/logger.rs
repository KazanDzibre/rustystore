use chrono::Local;
use log::{LevelFilter, Log, Metadata, Record};
use std::fs::File;
use std::io::Write;
use std::sync::Mutex;

/// MultiLogger logs to both a file and the console (stderr)
pub struct MultiLogger {
    pub file: Mutex<File>,
    pub inner: env_logger::Logger,
}

impl Log for MultiLogger {
    fn enabled(&self, metadata: &Metadata) -> bool {
        self.inner.enabled(metadata)
    }

    fn log(&self, record: &Record) {
        if self.enabled(record.metadata()) {
            let line = format!(
                "{} [{}] {}\n",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            );

            if let Ok(mut file) = self.file.lock() {
                let _ = file.write_all(line.as_bytes());
            }

            self.inner.log(record);
        }
    }

    fn flush(&self) {
        if let Ok(mut file) = self.file.lock() {
            let _ = file.flush();
        }
        self.inner.flush();
    }
}

/// Initializes the logging system.
/// Logs go to both a file and stderr, with level INFO.
pub fn init_logger(log_file_path: &std::path::Path) {
    let file = File::create(log_file_path)
        .or_else(|_| File::open(log_file_path))
        .expect("Failed to open log file");

    let logger = env_logger::Builder::new()
        .format(|buf, record| {
            writeln!(
                buf,
                "{} [{}] {}",
                Local::now().format("%Y-%m-%d %H:%M:%S"),
                record.level(),
                record.args()
            )
        })
        .target(env_logger::Target::Stderr)
        .filter_level(LevelFilter::Info)
        .build();

    log::set_boxed_logger(Box::new(MultiLogger {
        file: Mutex::new(file),
        inner: logger,
    }))
    .unwrap();

    log::set_max_level(LevelFilter::Info);
}
