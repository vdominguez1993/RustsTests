use crate::database;
use crate::log::*;
use std::fs;
use std::io::prelude::*;
use std::net::TcpListener;
use std::net::TcpStream;
use std::str;
use std::thread;
use threadpool::ThreadPool;

type ResponseHandler = fn(&str) -> Vec<u8>;

const THREADS_IN_POOL: usize = 4;
const IP: &str = "0.0.0.0:7878";
const WEB_ROOT: &str = "../web/";
const GET_SERVICES: &'static [(&'static str, ResponseHandler)] =
    &[("get_temperature", get_temperature_handler)];

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

    let (method, path) = parse_method_and_path(&buffer);
    // By default use the 404 handler
    let mut handler: ResponseHandler = error_404;

    match &method[..] {
        "GET" => {
            // Protect agains path traversal issuing an error if '..' is foud
            if is_trying_to_traverse(&path) {
                handler = path_traversal_error;
            // If the method is a get and the path exists as a file
            } else if file_in_path(&path) {
                // Use the file server handler
                handler = file_server;
            } else {
                // NOTE this will asign to 404 if service is not found
                handler = get_service(&path);
            }
        }
        _ => println!("Unrecognized method {}", method),
    }

    let response = handler(&path);

    stream.write(&&response).unwrap();
    stream.flush().unwrap();
}

fn parse_method_and_path(buffer: &[u8]) -> (String, String) {
    let mut path = String::new();
    let mut method = String::new();
    let mut string_accumulated = String::new();
    let mut element = 0;

    // For each character
    for x in buffer {
        match *x as char {
            // the only interesting thing is the first line
            '\r' => break,
            // The method will be the first and then the URL
            ' ' => {
                element = element + 1;
                match element {
                    1 => method = string_accumulated.clone(),
                    // When we have found the path everything is done
                    2 => {
                        path = string_accumulated.clone();
                        break;
                    }
                    _ => {}
                }
                string_accumulated.clear();
            }
            // Store the interesting char
            _ => string_accumulated.push(*x as char),
        };
    }

    // Remove the leading '/' if there is a path, if not,
    // The user is refering to index.html
    let return_path: String = match path {
        _ if path == "/" => String::from("index.html"),
        _ => String::from(&path[1..]),
    };

    (method, return_path)
}

fn file_in_path(path_to_check: &str) -> bool {
    let root_path = std::path::Path::new(WEB_ROOT);
    let full_path_check = root_path.join(path_to_check);

    // Check if file exists
    match fs::metadata(full_path_check) {
        // Return if it is a file
        Ok(file_handler) => file_handler.is_file(),
        // False if file does not exist or can't be opened
        Err(_) => false,
    }
}

fn get_service(path_to_check: &str) -> ResponseHandler {
    let mut retval: ResponseHandler = error_404;
    for &(name, handler) in GET_SERVICES {
        if &path_to_check == &name {
            retval = handler;
            break;
        }
    }

    retval
}

fn get_temperature_handler(_file_path: &str) -> Vec<u8> {
    let mut response: Vec<u8> = "HTTP/1.1 200 OK\r\n\r\n".as_bytes().to_vec();
    let temperature_record = database::get_last_entries(50).expect("Error getting data");

    response.append(&mut temperature_record.as_bytes().to_vec());
    response
}

fn is_trying_to_traverse(path_to_check: &str) -> bool {
    path_to_check.contains("..")
}

fn file_server(file_path: &str) -> Vec<u8> {
    // Serve the specified file content with an OK response
    let root_path = std::path::Path::new(WEB_ROOT);
    let mut response: Vec<u8> = "HTTP/1.1 200 OK\r\n\r\n".as_bytes().to_vec();
    let mut file = fs::File::open(root_path.join(file_path)).unwrap();

    file.read_to_end(&mut response).unwrap();

    response
}

fn error_404(_file_path: &str) -> Vec<u8> {
    // Serve the 404.html to show an error to the user
    let root_path = std::path::Path::new(WEB_ROOT);
    let mut response: Vec<u8> = "HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes().to_vec();
    let mut file = fs::File::open(root_path.join("404.html")).unwrap();

    file.read_to_end(&mut response).unwrap();

    response
}

fn path_traversal_error(_file_path: &str) -> Vec<u8> {
    // Serve the 404.html to show an error to the user
    let root_path = std::path::Path::new(WEB_ROOT);
    let mut response: Vec<u8> = "HTTP/1.1 404 NOT FOUND\r\n\r\n".as_bytes().to_vec();
    let mut file = fs::File::open(root_path.join("404.html")).unwrap();

    file.read_to_end(&mut response).unwrap();

    response
}
