use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::Result;

fn handle_client(mut stream: TcpStream) {
    let req = & mut [0u8; 4];
    stream.read(req);
    stream.write(req);
}

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;

    // accept connections and process them serially
    for stream in listener.incoming() {
        handle_client(stream?);
    }
    Ok(())
}