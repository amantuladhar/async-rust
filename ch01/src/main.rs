use std::arch::asm;

fn main() {
    let t = 100;
    let t_ptr: *const usize = &t;
    let x = deref(t_ptr);
    println!("{x}")
}

#[cfg(target_arch = "aarch64")]
fn deref(ptr: *const usize) -> usize {
    let mut res: usize = 0;
    unsafe {
        asm!("ldr {0}, [{1}]", out(reg) res, in(reg) ptr, options(nostack))
    };
    res
}

#[cfg(target_arch = "x86_64")]
fn deref(ptr: *const usize) -> usize {
    let mut res: usize;
    unsafe {
        asm!{"mov {0} [{1}]", out(reg) res, in(reg) ptr}
    }
    res
}