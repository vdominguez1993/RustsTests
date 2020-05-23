mod log;

use log::*;

fn main() {
    println!("Hello World");

    log::init();

    debug!("Que paishaa");
}
