// use nix::unistd::{fork, ForkResult};

// fn main() {
//     match fork() {
//         Ok(ForkResult::Parent { child, .. }) => {
//             println!("Continuing execution in parent process, new child has pid: {}", child);
//         }
//         Ok(ForkResult::Child) => println!("I'm a new child process"),
//         Err(_) => println!("Fork failed"),
//     }
// }
#![feature(asm)]

use std::io::Write;
use std::process::{Command, Stdio};

fn main() {
    let buf = &[1u8; 4];

    let mut diffs: [u64; 10000] = [0; 10000];
    let mut diff_early: u64 = 0;
    let mut diff_late: u64 = 0;
    unsafe {
        for i in 0..diffs.len() {
            let mut child = Command::new("cat")
                .stdin(Stdio::piped())
                .stdout(Stdio::piped())
                .spawn()
                .expect("Failed to spawn child process");

            {
                let mut stdin = child.stdin.as_mut().expect("Failed to open stdin");
                asm!("
                    rdtscp\n
                    shl rdx, 32\n
                    or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

                stdin.write_all(buf).expect("Failed to write to stdin");
            }

            let output = child.wait_with_output().expect("Failed to read stdout");
            asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n
                ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

            diffs[i] =  diff_late - diff_early;

            // assert_eq!(output.stdout, buf);
        }
    }

    let mut min = 0;
    for i in 0..diffs.len() {
        print!("{:?}, ", diffs[i]);
        if  min == 0 || diffs[i] < min {
            min = diffs[i];
        }
    }

    println!("min = {}", min);
    
}