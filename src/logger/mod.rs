use chrono::Local;
use std::fs::OpenOptions;
use std::io::Write;

pub struct Logger {
    file: Option<std::fs::File>,
}

impl Logger {
    pub fn new() -> Self {
        let file = OpenOptions::new()
            .create(true)
            .append(true)
            .open(".controlllog")
            .ok();
        Logger { file }
    }

    pub fn log_command(&mut self, command: &str) {
        let timestamp = Local::now().format("%Y-%m-%d %H:%M:%S");
        let log_line = format!("[{}] {}\n", timestamp, command);
        if let Some(file) = &mut self.file {
            let _ = file.write_all(log_line.as_bytes());
            let _ = file.flush();
        } else {
            // Если файл не открылся, пишем в stderr (но лучше не засорять терминал)
            eprintln!("Logging failed, but command executed: {}", command);
        }
    }
}