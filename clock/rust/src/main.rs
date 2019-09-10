#![feature(asm)]

fn main() {
    let mut diffs: [u64; 1000000] = [0; 1000000];
    let mut k = 0;
    let mut count: u64 = 10;
    let mut diff_early: u64 = 0;
    let mut diff_late: u64 = 0;

    unsafe {
        for j in 0..diffs.len() {
                asm!("
                cpuid\n
                rdtscp\n
                #shl rdx, 32\n
                #add rdx, rax\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

                /*asm!("
                xor rax, rax\n
                xor rbx, rbx":::"rax", "rbx":"intel");*/
                for k1 in 0..10000 {
                    asm!("mov rax, $1\n
                          add rax, 1\n": "={rax}"(count) : "r"(count): "rax", "memory" : "volatile", "intel");
                }

                asm!("
                rdtscp\n
                #shl rdx, 32\n
                #add rdx, rax\n
                ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

                diffs[j] =  diff_late - diff_early;
                asm!("cpuid":::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");
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
