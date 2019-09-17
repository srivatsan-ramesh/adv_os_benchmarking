use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::{Read};
use std::io::Result;

const SIZE : usize = 256*1024;

fn main() -> Result<()> {
    let listener = TcpListener::bind("127.0.0.1:8080")?;
    let req = & mut [0u8; SIZE];
    let mut bytes_written : usize = 0;
    let mut bytes_read : usize = 0;

    // accept connections and process them serially
    for streamt in listener.incoming() {
        let mut stream: TcpStream = streamt?;

        bytes_read = 0;
        let mut c1 = stream.read(&mut req[bytes_read..])?;

        while c1 > 0 {
            bytes_read += c1;
            if bytes_read >= SIZE {
                break;
            }
            c1 = stream.read(&mut req[bytes_read..])?;
        }
        
        bytes_written = 0;
        let mut c = stream.write(&req[bytes_written..])?;

        while c > 0 {
            bytes_written += c;
            if bytes_written >= SIZE {
                break;
            }
            c = stream.write(&req[bytes_written..])?;
        }
    }
    Ok(())
}