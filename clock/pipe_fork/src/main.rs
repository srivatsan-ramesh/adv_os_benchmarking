#![feature(asm)]
extern crate libc;

use std::env;
use std::process;

use libc::{fork, socketpair, close, write, read, PF_LOCAL, SOCK_STREAM, c_int, c_void};

fn prefetch(data: &mut [u8]) {
    for i in 0..data.len() {
        data[i] += 1;
    }

    for j in 0..data.len() {
        data[j] += 1;
    }
}


fn main() {

    let SIZE = match env::args().skip(1).next() {
        Some(num) => {
            let n = num.to_string();
            n.parse::<usize>().unwrap()
        },
        None => {
            println!("Invalid args -- SIZE");
            process::exit(1);           
        }
    };

    /*
    let num_iters = match env::args().next() {
        Some(num) => {
            let n = num.to_string();
            n.parse::<usize>().unwrap_or(10)
        },
        None => {
            println!("Invalid args -- num_iters");
            process::exit(1);
        }
    };*/


    let mut pipe_fds = [0; 2];
    let mut data = vec![0; SIZE];
    let mut diff_early: u64 = 0;
    let mut diff_late: u64 = 0;
    let mut min: u64 = 1000000000;

    let parent_fd = 0;
    let child_fd = 1;

    prefetch(&mut data);
    unsafe {
        if socketpair(PF_LOCAL, SOCK_STREAM, 0, pipe_fds.as_mut_ptr() as *mut c_int) == -1 {
            println!("socketpair failed!");
            process::exit(1);
        }
            if fork() == 0 {
                //Warm-up before performing the actual measurement
                for times in 0..1000001 {
                    let mut total = SIZE;
                    let mut i = 0;
                    let mut c = read(pipe_fds[child_fd], (data.as_mut_ptr() as *mut c_void).offset(i), total);
                    while c > 0 {
                        total = total - c as usize;
                        i += c;
                        c = read(pipe_fds[child_fd], (data.as_mut_ptr() as *mut c_void).offset(i), total);
                    }
                    assert!(total == 0);

                    total = SIZE;
                    i = 0;
                    c = write(pipe_fds[child_fd], (data.as_ptr() as *const c_void).offset(i), total);
                    while c > 0 {
                        total = total - c as usize;
                        i += c;
                        c = write(pipe_fds[child_fd], (data.as_ptr() as *const c_void).offset(i), total);
                    }
                

                    //Perform the round trip that is measured by the parent
                    let mut total = SIZE;
                    let mut i = 0;
                    let mut c = read(pipe_fds[child_fd], (data.as_mut_ptr() as *mut c_void).offset(i), total);
                    while c > 0 {
                        total = total - c as usize;
                        i += c;
                        c = read(pipe_fds[child_fd], (data.as_mut_ptr() as *mut c_void).offset(i), total);
                    }
                    assert!(total == 0);

                    total = SIZE;
                    i = 0;
                    c = write(pipe_fds[child_fd], (data.as_ptr() as *const c_void).offset(i), total);
                    while c > 0 {
                        total = total - c as usize;
                        i += c;
                        c = write(pipe_fds[child_fd], (data.as_ptr() as *const c_void).offset(i), total);
                    }
                    assert!(total == 0);
                }
                assert!(close(pipe_fds[child_fd]) == 0);
                assert!(close(pipe_fds[parent_fd]) == 0);
            
            } else {
                for times in 0..1000000 {
                    asm!("
                        rdtscp\n
                        shl rdx, 32\n
                        or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");
                    let mut total = SIZE;
                    let mut i = 0;
                    let mut c = write(pipe_fds[parent_fd], (data.as_ptr() as *const c_void).offset(i), total);
                    while c > 0 {
                        total = total - c as usize;
                        c = write(pipe_fds[parent_fd], (data.as_ptr() as *const c_void).offset(i), total);
                    }
                    assert!(total == 0);

                    total = SIZE;
                    i = 0;
                    let mut c = read(pipe_fds[parent_fd], (data.as_mut_ptr() as *mut c_void).offset(i), total);
                    while c > 0 {
                        total = total - c as usize;
                        c = read(pipe_fds[parent_fd], (data.as_mut_ptr() as *mut c_void).offset(i), total);
                    }
                    assert!(total == 0);
                    //read(pipe_fds[parent_fd], data.as_mut_ptr() as *mut c_void, 512 * 1024);

                    asm!("
                        rdtscp\n
                        shl rdx, 32\n
                        or rax, rdx\n
                        ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");
                    if (diff_late - diff_early) < min {
                        min = diff_late - diff_early;
                    }
                }
                println!("{:?} nanoseconds", (min as f64/3.2) / 2.0);
            }
    }
}
