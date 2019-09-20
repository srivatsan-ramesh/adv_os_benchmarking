use std::net::UdpSocket;

use std::env;
use std::process;

const ITERATIONS : usize = 100000;

fn main() -> std::io::Result<()> {
    {
        let sizes = vec![16*1024];

        let from = match env::args().skip(1).next() {
            Some(num) => {
                num.to_string()
            },
            None => {
                println!("Invalid args");
                process::exit(1);
            }
        };

        let to = match env::args().skip(2).next() {
            Some(num) => {
                num.to_string()
            },
            None => {
                println!("Invalid args");
                process::exit(1);
            }
        };

        let socket = UdpSocket::bind(from)?;

        let mut bytes_written : usize;
        let mut bytes_read : usize;

        for i in 0..sizes.len() {
            let mut buf = vec![0; sizes[i]];
            for _j in 0..ITERATIONS {

                bytes_written = 0;
                bytes_read = 0;

                let (mut c, _src) = socket.recv_from(&mut buf[bytes_read..])?;

                while c > 0 {
                    bytes_read += c;
                    if bytes_read >= sizes[i] {
                        break;
                    }
                    let (p, _s) = socket.recv_from(&mut buf[bytes_read..])?;
                    c = p;
                }

                c = socket.send_to(&mut buf[bytes_written..], &to)?;

                while c > 0 {
                    bytes_written += c;
                    if bytes_written >= sizes[i] {
                        break;
                    }
                    c = socket.send_to(&mut buf[bytes_written..], &to)?;
                }
            }
        }
        
    } // the socket is closed here
    Ok(())
}