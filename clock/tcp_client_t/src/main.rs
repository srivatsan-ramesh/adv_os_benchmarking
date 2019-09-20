#![feature(asm)]
use std::io::prelude::*;
use std::io::{Read};
use std::net::TcpStream;

use std::env;
use std::process;

const ITERATIONS : usize = 100000;

fn main() -> std::io::Result<()> {

    let sizes = vec![4, 16, 64, 256, 1024, 4096, 16*1024, 64*1024];

    let address = match env::args().skip(1).next() {
        Some(num) => {
            num.to_string()
        },
        None => {
            println!("Invalid args");
            process::exit(1);
        }
    };

    let buf = & mut vec![0u8; 1];

    let mut diffs = vec![0; sizes.len()];
    let mut diff_early: u64;
    let mut diff_late: u64;

    let mut stream = TcpStream::connect(address)?;
    stream.set_nodelay(true).expect("set_nodelay call failed");

    unsafe {
        for i in 0..sizes.len() {
            let mut bytes_written : usize;
            let mut bytes_read : usize;
            let data = & vec![1u8; sizes[i]];

           asm!("
            rdtscp\n
            shl rdx, 32\n
            or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

            for _j in 0..ITERATIONS {
                bytes_written = 0;
                let mut c = stream.write(&data[bytes_written..])?;

                while c > 0 {
                    bytes_written += c;
                    if bytes_written >= sizes[i] {
                        break;
                    }
                    c = stream.write(&data[bytes_written..])?;
                }
            }
            
            bytes_read = 0;
            let mut c1 = stream.read(&mut buf[bytes_read..])?;

            while c1 > 0 {
                
                bytes_read += c1;
                if bytes_read >= 1 {
                    break;
                }
                c1 = stream.read(&mut buf[bytes_read..])?;
            }
            
            asm!("
            rdtscp\n
            shl rdx, 32\n
            or rax, rdx\n
            ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

            diffs[i] =  diff_late - diff_early;

            let cycles : f64 = diffs[i] as f64/ITERATIONS as f64;
            let tpt = (sizes[i] as f64)*1000000000.0*3.2/cycles/1024.0/1024.0;
            println!("{} bytes, cycles={}, time={}, tpt={}", sizes[i], cycles, (cycles)/3.2, tpt);
        }
    }

    Ok(())
} // the stream is closed here