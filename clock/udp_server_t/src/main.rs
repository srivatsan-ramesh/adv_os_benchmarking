use std::net::UdpSocket;

use std::env;
use std::process;
use std::time::Duration;


const ITERATIONS : usize = 1000000;

fn main() -> std::io::Result<()> {
    {
        let sizes = vec![4, 16, 64, 256, 1024, 4096, 16*1024];
        let mut bytes_received = vec![0; sizes.len()];

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
        socket.set_read_timeout(Some(Duration::new(5, 0))).expect("set_read_timeout call failed");

        let mut bytes_written : usize;
        let mut bytes_read : usize;

        for i in 0..sizes.len() {
            let size = sizes[i];
            let mut buf = vec![0; size];
            let mut ack = vec![0; 1];

            bytes_written = 0;
            for _j in 0..ITERATIONS {
                bytes_read = 0;

                let mut c = 0;
                let mut err = 0;

                match socket.recv_from(&mut buf[bytes_read..]) {
                    Ok((ref mut count, _s)) => {
                        c = *count;
                    },
                    Err(e) => {
                        if e.kind() == std::io::ErrorKind::WouldBlock {
                            err = -1;
                        }
                    }
                }



                while c > 0 && err != -1 {
                    bytes_read += c;
                    if bytes_read >= size {
                        break;
                    }
                    match socket.recv_from(&mut buf[bytes_read..]) {
                        Ok((ref mut count, _s)) => {
                            c = *count;
                        },
                        Err(e) => {
                            if e.kind() == std::io::ErrorKind::WouldBlock {
                                err = -1;
                            }
                        }
                    }
                }

                bytes_received[i] += bytes_read;
                if err == -1 {
                    break;
                }
            }

            let mut c = socket.send_to(&mut ack[bytes_written..], &to)?;

            while c > 0 {
                bytes_written += c;
                if bytes_written >= 1 {
                    break;
                }
                c = socket.send_to(&mut ack[bytes_written..], &to)?;
            }
        }

        for i in 0..bytes_received.len() {
            println!("Sent {}, Received {}, Dropped {}", sizes[i]*ITERATIONS, bytes_received[i], sizes[i]*ITERATIONS - bytes_received[i]);
        }
    } // the socket is closed here
    Ok(())
}