use std::{
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

const ADDRESS: &str = "127.0.0.1:4221";
const OK: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";
const NOT_FOUND: &[u8] = b"HTTP/1.1 404 NotFound\r\n\r\n";

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
    let mut buffer = String::new();
    stream
        .read_to_string(&mut buffer)
        .expect("Failed to read to string buffer");

    match buffer.lines().next() {
        None => panic!("The message is empty"),
        Some(request_line) => {
            let response = handle_request(request_line);
            stream
                .write_all(response)
                .expect("Failed to write response");
        }
    }
}

fn handle_request(request_line: &str) -> &[u8] {
    let [method, path, ..]: [&str; 3] = request_line
        .split_whitespace()
        .collect::<Vec<_>>()
        .try_into()
        .expect("Failed to parse request line");

    if (method, path) == ("GET", "/") {
        OK
    } else {
        NOT_FOUND
    }
}
