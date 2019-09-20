use std::net::{TcpListener, TcpStream};
use std::io::prelude::*;
use std::io::{Read};
use std::io::Result;

use std::env;
use std::process;

const ITERATIONS : usize = 100000;

fn main() -> Result<()> {
    let sizes = vec![4, 16, 64, 256, 1024, 4096, 16*1024];

    let address = match env::args().skip(1).next() {
        Some(num) => {
            num.to_string()
        },
        None => {
            println!("Invalid args");
            process::exit(1);
        }
    };

    let listener = TcpListener::bind(address)?;
    let mut bytes_written : usize;
    let mut bytes_read : usize;

    // accept connections and process them serially
    for streamt in listener.incoming() {
        let mut stream: TcpStream = streamt?;
        stream.set_nodelay(true).expect("set_nodelay call failed");

        for i in 0..sizes.len() {
            let req = & mut vec![0u8; sizes[i]];
            for _j in 0..ITERATIONS {
                bytes_read = 0;
                
                let mut c1 = stream.read(&mut req[bytes_read..])?;

                while c1 > 0 {
                    bytes_read += c1;
                    if bytes_read >= sizes[i] {
                        break;
                    }
                    c1 = stream.read(&mut req[bytes_read..])?;
                }
                
                bytes_written = 0;
                let mut c = stream.write(&req[bytes_written..])?;

                while c > 0 {
                    bytes_written += c;
                    if bytes_written >= sizes[i] {
                        break;
                    }
                    c = stream.write(&req[bytes_written..])?;
                }
            }
        }
    }
    Ok(())
}