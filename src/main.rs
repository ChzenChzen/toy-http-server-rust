use std::{
    io::{BufRead, Read, Write},
    net::{TcpListener, TcpStream},
};

const ADDRESS: &str = "127.0.0.1:4221";
const OK: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND: &[u8] = b"HTTP/1.1 404 Not Found\r\n\r\n";

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
        .write_all(response)
        .expect("Failed to write response");
}

fn get_response(request_line: &str) -> &[u8] {
    let [_method, path, ..]: [&str; 3] = request_line
        .split_whitespace()
        .collect::<Vec<_>>()
        .try_into()
        .expect("Failed to parse request line");

    match path {
        "/" => OK,
        _ => NOT_FOUND,
    }
}
