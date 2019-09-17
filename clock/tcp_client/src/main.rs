#![feature(asm)]
use std::io::prelude::*;
use std::io::{Read};
use std::net::TcpStream;

const SIZE : usize = 256*1024;
const ITERATIONS : usize = 1000;

fn main() -> std::io::Result<()> {

    let data = &[1u8; SIZE];
    let buf = & mut [0u8; SIZE];

    let mut diffs: [u64; ITERATIONS] = [0; ITERATIONS];
    let mut diff_early: u64 = 0;
    let mut diff_late: u64 = 0;


    unsafe {
        for j in 0..diffs.len() {
                let mut stream = TcpStream::connect("127.0.0.1:8080")?;
                stream.set_nodelay(true).expect("set_nodelay call failed");
                stream.set_nonblocking(false)?;
                let mut bytes_written : usize = 0;
                let mut bytes_read : usize = 0;
                asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

                bytes_written = 0;
                let mut c = stream.write(&data[bytes_written..])?;

                while c > 0 {
                    bytes_written += c;
                    if bytes_written >= SIZE {
                        break;
                    }
                    c = stream.write(&data[bytes_written..])?;
                }
                bytes_read = 0;
                let mut c1 = stream.read(&mut buf[bytes_read..])?;

                while c1 > 0 {
                   
                    bytes_read += c1;
                    if bytes_read >= SIZE {
                        break;
                    }
                    c1 = stream.read(&mut buf[bytes_read..])?;
                }
                
                asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n
                ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

                diffs[j] =  diff_late - diff_early;
        }


    }

    let mut min = 0;
    for i in 0..diffs.len() {
        print!("{:?}, ", diffs[i]);
        if min == 0 || diffs[i] < min {
            min = diffs[i];
        }

    }    

    println!("min = {}", min);

    Ok(())
} // the stream is closed here