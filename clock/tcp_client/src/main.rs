#![feature(asm)]
use std::io::prelude::*;
use std::net::TcpStream;
use std::str;

fn main() -> std::io::Result<()> {
    let data = &[1u8; 4];
    let buf = & mut [0u8; 4];

    let mut diffs: [u64; 10000] = [0; 10000];
    let mut count: u64 = 10;
    let mut diff_early: u64 = 0;
    let mut diff_late: u64 = 0;

    unsafe {
        for j in 0..diffs.len() {
                let mut stream = TcpStream::connect("127.0.0.1:8080")?;
                stream.set_nodelay(true).expect("set_nodelay call failed");

                asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

                stream.write(data)?;
                stream.read(buf)?;

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