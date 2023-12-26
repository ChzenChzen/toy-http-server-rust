use std::{
    error::Error,
    io::{Read, Write},
    net::{TcpListener, TcpStream},
};

type Result<T> = std::result::Result<T, Box<dyn Error>>;

const ADDRESS: &str = "127.0.0.1:4221";
const RESPONSE_OK: &[u8] = b"HTTP/1.1 200 OK\r\n\r\n";

fn main() -> Result<()> {
    println!("Listening on {ADDRESS}");
    let listener = TcpListener::bind(ADDRESS)?;

    for stream in listener.incoming() {
        match stream {
            Ok(mut stream) => {
                handle_incoming(&mut stream)?;
            }
            Err(e) => {
                println!("error: {}", e);
            }
        }
    }

    Ok(())
}

fn handle_incoming(stream: &mut TcpStream) -> Result<()> {
    let mut string = String::new();
    stream.read_to_string(&mut string)?;
    println!("accepted new connection, the message is `{string}`");
    string.clear();

    println!("sending `OK` response");
    stream.write_all(RESPONSE_OK)?;

    Ok(())
}
