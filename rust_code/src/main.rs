use std::{thread, time};

#[macro_use]
pub mod log;
pub mod database;
mod web_server;

use log::*;

const DATABASE_SLEEP: time::Duration = time::Duration::from_secs(5);

fn main() {
    log_init();

    web_server::start();
    loop {
        database::update_data();
        thread::sleep(DATABASE_SLEEP);
    }
}
