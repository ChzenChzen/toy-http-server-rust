use std::{
    io::{BufRead, Read, Write},
    net::{TcpListener, TcpStream},
    path::PathBuf,
};

const ADDRESS: &str = "127.0.0.1:4221";
const OK: &str = "HTTP/1.1 200 OK";
const CREATED: &str = "HTTP/1.1 201 Created";
const NOT_FOUND: &str = "HTTP/1.1 404 Not Found";
const TEXT_PLAIN: &str = "Content-Type: text/plain";
const OCTET_STREAM: &str = "Content-Type: application/octet-stream";
const CONTENT_LENGTH: &str = "Content-Length";
const USER_AGENT_PATH: &str = "user-agent";
const ECHO_PATH: &str = "echo";
const FILES_PATH: &str = "files";
const GET: &str = "GET";
const POST: &str = "POST";
const DIRECTORY_FLAG: &str = "--directory";

fn main() {
    println!("Listening on {ADDRESS}");
    let listener = TcpListener::bind(ADDRESS).expect("Failed to bind");
    for stream in listener.incoming() {
        match stream {
            Ok(stream) => {
                println!("Handling incoming");
                std::thread::spawn(move || handle_incoming(stream));
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }
}

fn handle_incoming(mut stream: TcpStream) {
    let mut buffer = [0; 1024];
    stream
        .read(&mut buffer)
        .expect("Failed to read to string buffer");

    let response = build_response(&buffer);

    stream
        .write_all(response.as_bytes())
        .expect("Failed to write response");
}

fn build_response(buffer: &[u8]) -> String {
    let mut request = buffer.lines().filter_map(Result::ok);

    let request_line = request.next().expect("Header is empty");
    let [method, path, ..] = parse_request_line(&request_line);

    let path_parts: Vec<_> = path.splitn(3, '/').collect();
    match (method, path_parts.as_slice()) {
        (GET, &["", FILES_PATH, filename]) => get_file_response(filename),
        (POST, &["", FILES_PATH, filename]) => {
            let body = request.skip(4).collect::<String>();
            let body = body.trim_end_matches(0 as char);
            post_file_response(filename, body)
        }
        (_, &["", USER_AGENT_PATH]) => get_user_agent_response(&mut request),
        (_, &["", ECHO_PATH, rest]) => format!(
            "{OK}\r\n{TEXT_PLAIN}\r\n{CONTENT_LENGTH}: {content_length}\r\n\r\n{rest}",
            content_length = rest.len(),
        ),
        (_, &["", ""]) => format!("{OK}\r\n\r\n"),
        _ => format!("{NOT_FOUND}\r\n\r\n"),
    }
}

fn post_file_response(filename: &str, content: impl AsRef<[u8]>) -> String {
    let (flag, directory) = parse_arguments();
    save_file(filename, &flag, &directory, content);
    format!("{CREATED}\r\n\r\n")
}

fn save_file(filename: &str, flag: &str, directory: &str, content: impl AsRef<[u8]>) {
    match (flag, directory) {
        (DIRECTORY_FLAG, directory) => {
            let path = PathBuf::from(directory).join(filename);
            std::fs::write(path, content).expect("Failed to write file");
        }
        _ => panic!("Invalid flag for executable"),
    }
}

fn get_file_response(filename: &str) -> String {
    let (flag, directory) = parse_arguments();
    match get_file(filename, &flag, &directory) {
        Ok(file) => {
            let content = String::from_utf8_lossy(&file);
            format!(
                "{OK}\r\n{OCTET_STREAM}\r\n{CONTENT_LENGTH}: {content_length}\r\n\r\n{content}",
                content_length = content.len()
            )
        }
        Err(_) => format!("{NOT_FOUND}\r\n\r\n"),
    }
}

fn parse_arguments() -> (String, String) {
    let arguments: Vec<_> = std::env::args().collect();
    let flag = arguments.get(1).cloned().expect("Failed to get flag");
    let directory = arguments.get(2).cloned().expect("Failed to get directory");
    (flag, directory)
}

fn get_file(filename: &str, flag: &str, directory: &str) -> std::io::Result<Vec<u8>> {
    match (flag, directory) {
        (DIRECTORY_FLAG, directory) => {
            let path = std::env::current_dir()
                .expect("Failed to get current dir")
                .join(directory)
                .join(filename);
            std::fs::read(path)
        }
        _ => panic!("Invalid flag for executable"),
    }
}

fn parse_request_line(request_line: &str) -> [&str; 3] {
    request_line
        .split_whitespace()
        .collect::<Vec<_>>()
        .try_into()
        .expect("Failed to parse request line")
}

fn get_user_agent_response(mut request: impl Iterator<Item = String>) -> String {
    let user_agent_line = request.nth(1).expect("Failed to get user agent line");

    let user_agent = user_agent_line
        .split_once(':')
        .expect("Failed to split user agent line")
        .1
        .trim();

    format!(
        "{OK}\r\n{TEXT_PLAIN}\r\n{CONTENT_LENGTH}: {content_length}\r\n\r\n{user_agent}",
        content_length = user_agent.len(),
    )
}
