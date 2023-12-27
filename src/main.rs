use std::{
    io::{BufRead, Read, Write},
    net::{TcpListener, TcpStream},
};

const ADDRESS: &str = "127.0.0.1:4221";
const OK: &str = "HTTP/1.1 200 OK";
const CONTENT_TYPE: &str = "Content-Type: text/plain";

fn main() {
    println!("Listening on {ADDRESS}");
    let listener = TcpListener::bind(ADDRESS).expect("Failed to bind");

    for connection in listener.incoming() {
        match connection {
            Ok(mut stream) => {
                println!("Handling incoming");
                handle_incoming(&mut stream);
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_incoming(stream: &mut TcpStream) {
    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .expect("Failed to read to string buffer");

    let request_line = buffer
        .lines()
        .next()
        .expect("Header is empty")
        .expect("Failed to convert buffer into string");

    let response = get_response(&request_line);

    stream
        .write_all(response.as_bytes())
        .expect("Failed to write response");
}

fn get_response(request_line: &str) -> String {
    let [_method, path, ..]: [&str; 3] = request_line
        .split_whitespace()
        .collect::<Vec<_>>()
        .try_into()
        .expect("Failed to parse request line");

    let last_path_part = path.split_once('/').expect("Failed to split path").1;
    let content_length = last_path_part.len();

    format!("{OK}\r\n{CONTENT_TYPE}\r\nContent-Length: {content_length}\r\n{last_path_part}\r\n")
}
