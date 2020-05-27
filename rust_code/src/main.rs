#[macro_use]
pub mod log;
mod web_server;

use log::*;
use std::env;

fn main() {
    log_init();

    println!("{}", env::current_dir().unwrap().as_path().display());
    web_server::init();

    loop {}
}
