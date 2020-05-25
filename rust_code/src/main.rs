#[macro_use]
pub mod log;
mod web_server;

use log::*;

fn main() {
    log_init();

    web_server::init();
}
