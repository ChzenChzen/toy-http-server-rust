use std::{
    io::{BufRead, Read, Write},
    net::{TcpListener, TcpStream},
};

const ADDRESS: &str = "127.0.0.1:4221";
const OK: &str = "HTTP/1.1 200 OK";
const NOT_FOUND: &str = "HTTP/1.1 404 Not Found";
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

    let response = get_response(&buffer);

    stream
        .write_all(response.as_bytes())
        .expect("Failed to write response");
}

const USER_AGENT_PATH: &str = "user-agent";
const ECHO_PATH: &str = "echo";

fn get_response(buffer: &[u8]) -> String {
    let mut request = buffer.lines().filter_map(Result::ok);

    let request_line = request.next().expect("Header is empty");

    let [_method, path, ..]: [&str; 3] = request_line
        .split_whitespace()
        .collect::<Vec<_>>()
        .try_into()
        .expect("Failed to parse request line");

    let parts: Vec<_> = path.splitn(3, '/').collect();
    match parts.as_slice() {
        &["", USER_AGENT_PATH] => {
            let user_agent_line = request.nth(1).expect("Failed to get user agent line");

            let user_agent = user_agent_line
                .split_once(':')
                .expect("Failed to split user agent line")
                .1
                .trim();

            format!(
                "{OK}\r\n{CONTENT_TYPE}\r\nContent-Length: {content_length}\r\n\r\n{user_agent}",
                content_length = user_agent.len(),
            )
        }
        &["", ECHO_PATH, rest] => format!(
            "{OK}\r\n{CONTENT_TYPE}\r\nContent-Length: {content_length}\r\n\r\n{rest}",
            content_length = rest.len(),
        ),
        &["", ""] => format!("{OK}\r\n\r\n"),
        _ => format!("{NOT_FOUND}\r\n\r\n"),
    }
}
