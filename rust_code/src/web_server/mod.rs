use crate::log::*;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::thread;
use threadpool::ThreadPool;

const THREADS_IN_POOL: usize = 4;
const IP: &str = "0.0.0.0:7878";
const WEB_ROOT: &str = "../web/";

pub fn start() {
    thread::spawn(|| server_thread());
}

fn server_thread() {
    let listener = TcpListener::bind(IP).unwrap();
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

    let pair = parse_method_and_path(&buffer);
    println!("method {} path {}", pair.0, pair.1);
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

fn parse_method_and_path(buffer: &[u8]) -> (String, String) {
    let mut path = String::new();
    let mut method = String::new();
    let mut string_accumulated = String::new();
    let mut element = 0;
    for x in buffer {
        match *x as char {
            '\r' => break,
            ' ' => {
                element = element + 1;
                match element {
                    1 => method = string_accumulated.clone(),
                    2 => path = string_accumulated.clone(),
                    _ => {}
                }
                println!("Acumulated {}", string_accumulated);
                string_accumulated.clear();
            }
            _ => string_accumulated.push(*x as char),
        };
    }

    (method, path)
}
