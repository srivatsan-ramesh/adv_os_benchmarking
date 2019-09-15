#![feature(asm)]
use std::net::UdpSocket;

fn main() -> std::io::Result<()> {
    let mut diffs: [u64; 1000] = [0; 1000];
    let mut count: u64 = 10;
    let mut diff_early: u64 = 0;
    let mut diff_late: u64 = 0;

    let mut buf = [21u8; 4];

    unsafe {
        for j in 0..diffs.len() {
                let mut socket = UdpSocket::bind("127.0.0.1:8082")?;

                asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

                socket.send_to(&mut buf, "127.0.0.1:8081")?;
                socket.recv_from(&mut buf)?;

                asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n
                ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

                diffs[j] =  diff_late - diff_early;
                println!("{}", buf[0]);
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
}