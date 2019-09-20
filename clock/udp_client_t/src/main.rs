#![feature(asm)]
use std::net::UdpSocket;

use std::env;
use std::process;
use std::time::Duration;


const ITERATIONS : usize = 1000000;

fn main() -> std::io::Result<()> {

    let sizes = vec![4, 16, 64, 256, 1024, 4096, 16*1024];

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

    let mut diffs = vec![0; sizes.len()];
    let mut diff_early: u64;
    let mut diff_late: u64;

    let mut bytes_written : usize;
    let mut bytes_read : usize;
    
    let socket = UdpSocket::bind(from)?;
    socket.set_read_timeout(Some(Duration::new(10, 0))).expect("set_read_timeout call failed");

    unsafe {
        for j in 0..sizes.len() {
            let size = sizes[j];

            let buf = & mut vec![21u8; size];
            let ack = & mut vec![0u8; 1];

            bytes_read = 0;

            asm!("
            rdtscp\n
            shl rdx, 32\n
            or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

            for _i in 0..ITERATIONS {
                bytes_written = 0;

                let mut c = socket.send_to(&mut buf[bytes_written..], &to)?;

                while c > 0 {
                    bytes_written += c;
                    if bytes_written >= size {
                        break;
                    }
                    c = socket.send_to(&mut buf[bytes_written..], &to)?;
                }
            }

            let mut c = 0;
            let mut err = 0;

            match socket.recv_from(&mut ack[bytes_read..]) {
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
                if bytes_read >= 1 {
                    break;
                }
                match socket.recv_from(&mut ack[bytes_read..]) {
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


            asm!("
            rdtscp\n
            shl rdx, 32\n
            or rax, rdx\n
            ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

            diffs[j] =  diff_late - diff_early;

            let cycles : f64 = diffs[j] as f64/ITERATIONS as f64;
            let tpt = (sizes[j] as f64)*1000000000.0*3.2/cycles/1024.0/1024.0;
            println!("{} bytes, cycles={}, time={}, tpt={}, timeout={}", sizes[j], cycles, (cycles)/3.2, tpt, err);
        }


    }

    Ok(())
}