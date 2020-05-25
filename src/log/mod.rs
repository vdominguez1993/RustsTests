extern crate chrono;

use chrono::{Datelike, Timelike, Utc};
use std::fs;
use std::io::Write;
use std::path::Path;

const LOGS_PATH: &str = "./logs";

#[allow(dead_code)]
pub enum LogLevel {
    DEBUG,
    INFO,
    WARNING,
    ERROR
}

#[macro_export]
macro_rules! debug {
    ($($x:expr),*) =>
    {
        let formatted_input = format!($($x,)*);
        write_to_log(LogLevel::DEBUG, formatted_input);
    }
}

pub fn log_init() {
    create_log_folder();
}

fn create_log_folder() {
    let logs_path: &Path = &Path::new(LOGS_PATH);

    if let Err(_err) = fs::create_dir_all(logs_path) {
        panic!("Problem for folder: {} {}", logs_path.display(), _err);
    }
}

pub fn write_to_log(level : LogLevel, text: String) {
    let now = Utc::now();
    let date = format!("{}-{:02}-{:02}.log", now.year(), now.month(), now.day());
    let hour = format!("{:02}:{:02}:{:02}", now.hour(), now.minute(), now.second());
    let path_log = Path::new(LOGS_PATH).join(date);
    let level_info = match level {
        LogLevel::DEBUG => "DEBUG",
        LogLevel::INFO => "INFO",
        LogLevel::WARNING => "WARNING",
        LogLevel::ERROR => "ERROR",
    };

    let text_to_write = format!("{} {} {}\r\n", level_info, hour, text);


    let mut file = fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path_log)
        .expect("Couldn't open file");

    file.write(text_to_write.as_bytes())
        .expect("Coudln't write file");
}
