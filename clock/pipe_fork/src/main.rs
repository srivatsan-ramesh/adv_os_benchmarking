#![feature(asm)]
extern crate libc;

use libc::{fork, socketpair, write, read, PF_LOCAL, SOCK_STREAM, c_int, c_void};
const SIZE: usize = 4;

fn main() {

    let mut pipe_fds = [0; 2];
    let mut data = [0; SIZE];
    let mut diff_early: u64 = 0;
    let mut diff_late: u64 = 0;

    let parent_fd = 0;
    let child_fd = 1;

    unsafe {
        socketpair(PF_LOCAL, SOCK_STREAM, 0, pipe_fds.as_mut_ptr() as *mut c_int);
        if fork() == 0 {
            let mut total = SIZE;
            let mut c = read(pipe_fds[child_fd], data.as_mut_ptr() as *mut c_void, total);
            while c > 0 {
                total = total - c as usize;
                c = read(pipe_fds[child_fd], data.as_mut_ptr() as *mut c_void, total);
            }

            let mut total = SIZE;
            c = write(pipe_fds[child_fd], data.as_ptr() as *const c_void, total);
            while c > 0 {
                total = total - c as usize;
                c = write(pipe_fds[child_fd], data.as_ptr() as *const c_void, total);
            }
            
        } else {
            asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");
            let mut total = SIZE;
            let mut c = write(pipe_fds[parent_fd], data.as_ptr() as *const c_void, total);
            while c > 0 {
                total = total - c as usize;
                c = write(pipe_fds[parent_fd], data.as_ptr() as *const c_void, total);
            }

            let mut total = SIZE;
            let mut c = read(pipe_fds[parent_fd], data.as_mut_ptr() as *mut c_void, total);
            while c > 0 {
                total = total - c as usize;
                c = read(pipe_fds[parent_fd], data.as_mut_ptr() as *mut c_void, total);
            }

            //read(pipe_fds[parent_fd], data.as_mut_ptr() as *mut c_void, 512 * 1024);

            asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n
                ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");
            println!("{:?}", diff_late - diff_early);
        }
        
    }
}
