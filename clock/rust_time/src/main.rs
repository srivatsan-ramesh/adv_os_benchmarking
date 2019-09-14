#![feature(asm)]
extern crate libc;

use libc::{CLOCK_REALTIME, CLOCK_THREAD_CPUTIME_ID,clock_gettime, timespec};

fn main() {
    let mut diffs: [i64; 1000000] = [0; 1000000];
    let mut k = 0;
    let mut count: u64 = 10;
    let mut diff_early: u64 = 0;
    let mut diff_late: u64 = 0;


    for i in 0..diffs.len() {
        unsafe {
            let mut tspec1 = timespec { tv_sec: 0, tv_nsec: 0};
            let mut tspec2 = timespec { tv_sec: 0, tv_nsec: 0};
                
            let start = clock_gettime(CLOCK_REALTIME, &mut tspec1 as *mut timespec);
       
            for k1 in 0..0 {
                    asm!("mov rax, $1\n
                          add rax, 1\n": "={rax}"(count) : "r"(count): "rax", "memory" : "volatile", "intel");
            }
            
            let end = clock_gettime(CLOCK_REALTIME, &mut tspec2 as *mut timespec);
            assert!(tspec2.tv_sec >= tspec1.tv_sec);
            diffs[i] = (tspec2.tv_sec - tspec1.tv_sec) * 1000000000 + (tspec2.tv_nsec - tspec1.tv_nsec);
        }

    }

    let mut min = 1000000000;
    for i in 0..diffs.len() {
        print!("{:?}, ", diffs[i]);
        if diffs[i] < min {
            min = diffs[i];
        }

    }

    println!("count = {}, min = {}", count, min);


}
