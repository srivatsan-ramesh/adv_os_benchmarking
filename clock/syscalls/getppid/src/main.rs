#![feature(asm)]

fn main() {
    let mut diffs: [u64; 1000000] = [0; 1000000];
    let mut ppid: u64 = 0;
    let mut diff_early: u64 = 0;
    let mut diff_late: u64 = 0;

    unsafe {
        for j in 0..diffs.len() {
                asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n": "={rax}"(diff_early)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

                /*asm!("
                xor rax, rax\n
                xor rbx, rbx":::"rax", "rbx":"intel");*/
                for _k1 in 0..1 {
                    asm!("mov rax, 110\n
                          syscall\n": "={rax}"(ppid) :: "rax", "memory" : "volatile", "intel");
                }

                asm!("
                rdtscp\n
                shl rdx, 32\n
                or rax, rdx\n
                ": "={rax}"(diff_late)::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");

                diffs[j] =  diff_late - diff_early;
                //asm!("cpuid":::"rax", "rdx", "rcx", "rbx", "memory": "volatile", "intel");
        }


    }

    let mut min = 1000000000;
    for i in 0..diffs.len() {
        print!("{:?}, ", diffs[i]);
        if diffs[i] < min {
            min = diffs[i];
        }

    }

    println!("ppid = {}, min = {}", ppid, min);
}
