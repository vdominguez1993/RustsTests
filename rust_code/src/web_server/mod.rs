use std::net::TcpListener;
use crate::log::*;

pub fn init() {
    let listener = TcpListener::bind("127.0.0.1:7878").unwrap();

    debug!("ewiufhefwiu");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        println!("Connection established!");
    }
}