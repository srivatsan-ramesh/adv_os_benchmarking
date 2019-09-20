#![feature(asm)]
use std::net::UdpSocket;

use std::env;
use std::process;

const ITERATIONS : usize = 100000;

fn main() -> std::io::Result<()> {

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

    let mut diff_early: u64;
    let mut diff_late: u64;

    let mut bytes_written : usize;
    let mut bytes_read : usize;
    let socket = UdpSocket::bind(from)?;


    unsafe {
        for i in 0..sizes.len() {
            let mut buf = & mut vec![21u8; sizes[i]];
            let mut min_time : u64 = 0;
            for _j in 0..ITERATIONS {

                bytes_written = 0;
                bytes_read = 0;

                asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

                let mut c = socket.send_to(&mut buf, &to)?;

                while c > 0 {
                    bytes_written += c;
                    if bytes_written >= sizes[i] {
                        break;
                    }
                    c = socket.send_to(&mut buf, &to)?;
                }

                let (p, _s) = socket.recv_from(&mut buf[bytes_read..])?;
                c = p;

                while c > 0 {
                    bytes_read += c;
                    if bytes_read >= sizes[i] {
                        break;
                    }
                    let (q, _r) = socket.recv_from(&mut buf[bytes_read..])?;
                    c = q;
                }


                asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n
                ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

                let time =  diff_late - diff_early;
                if min_time == 0 || time < min_time {
                    min_time = time;
                }
                assert_eq!(bytes_read, sizes[i], "Not same {} bytes and {} bytes", bytes_read, sizes[i]);
            }
            println!("{} bytes, min_time {}", sizes[i], min_time as f64/6.4);
        }
    }

    Ok(())
}
