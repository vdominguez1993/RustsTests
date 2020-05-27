use crate::log::*;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::thread;
use threadpool::ThreadPool;

const THREADS_IN_POOL: usize = 4;
const WEB_ROOT: &str = "../web/";

pub fn init() {
    thread::spawn(|| server_thread());
}

fn server_thread() {
    let listener = TcpListener::bind("0.0.0.0:7878").unwrap();
    let pool = ThreadPool::new(THREADS_IN_POOL);

    debug!("Server start");

    for stream in listener.incoming() {
        let stream = stream.unwrap();

        pool.execute(|| {
            handle_connection(stream);
        });
    }
}

fn handle_connection(mut stream: TcpStream) {
    let mut buffer = [0; 512];

    stream.read(&mut buffer).unwrap();

    let trimmed_input = str::from_utf8(&buffer).unwrap().trim_matches(char::from(0));

    debug!("Incoming data from {}", stream.peer_addr().unwrap());
    debug!("Received data:\n{}", trimmed_input);

    let get = b"GET / HTTP/1.1\r\n";
    let (status_line, filename) = if buffer.starts_with(get) {
        ("HTTP/1.1 200 OK\r\n\r\n", "index.html")
    } else {
        ("HTTP/1.1 404 NOT FOUND\r\n\r\n", "404.html")
    };

    let root_path = std::path::Path::new(WEB_ROOT);
    println!("{}", root_path.join(filename).display());
    let contents = fs::read_to_string(root_path.join(filename)).unwrap();
    let response = format!("{}{}", status_line, contents);

    stream.write(response.as_bytes()).unwrap();
    stream.flush().unwrap();
}
