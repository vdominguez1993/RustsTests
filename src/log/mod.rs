extern crate chrono;

use chrono::{Datelike, Utc};
use std::fs;
use std::io::Write;
use std::path::Path;

const LOGS_PATH: &str = "logs";

#[macro_export]
macro_rules! debug {
    ($($x:expr),*) =>
    {
        let formatted_input = format!($($x,)*);
        write_to_log(formatted_input);
    }
}

pub fn init() {
    create_log_folder();

    debug!("Holi {}", 3);
}

fn create_log_folder() {
    let logs_path: &Path = &Path::new(LOGS_PATH);

    if let Err(_err) = fs::create_dir_all(logs_path) {
        panic!("Problem for folder: {} {}", logs_path.display(), _err);
    }
}

pub fn write_to_log(text: String) {
    let now = Utc::now();
    let date = format!("{}-{:02}-{:02}.log", now.year(), now.month(), now.day());
    let path_log = Path::new(LOGS_PATH).join(date);
    let text_to_write = format!("{}\r\n", text);

    let mut file = match fs::OpenOptions::new()
        .read(true)
        .append(true)
        .create(true)
        .open(path_log)
    {
        Ok(i) => i,
        Err(_err) => panic!("Something happened {}", _err),
    };

    if let Err(_err) = file.write(text_to_write.as_bytes()) {
        panic!("Something happened {}", _err);
    }
}
