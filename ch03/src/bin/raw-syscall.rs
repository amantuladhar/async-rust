use std::arch::asm;

fn main() {
    let message = String::from("Hello world from raw syscall!\n");
    syscall(message);
}

// ----------------------------------------------------------------------------
// Linux raw syscall
// ----------------------------------------------------------------------------

#[cfg(target_os = "linux")]
#[inline(never)]
fn syscall(message: String) {
    let msg_ptr = message.as_ptr();
    let len = message.len();

    unsafe {
        asm!(
        "mov rax, 1",      // system call 1 is write on Linux
        "mov rdi, 1",      // file handle 1 is stdout
        "syscall",         // call kernel, software interrupt
        in("rsi") msg_ptr, // address of string to output
        in("rdx") len,     // number of bytes
        out("rax") _, out("rdi") _, lateout("rsi") _, lateout("rdx") _
        );
    }
}

// ----------------------------------------------------------------------------
// macOS raw syscall when running intel CPU (x86_64 architecture)
// ----------------------------------------------------------------------------

#[cfg(all(target_os = "macos", target_arch = "x86_64"))]
#[inline(never)]
fn syscall(message: String) {
    let msg_ptr = message.as_ptr();
    let len = message.len();

    unsafe {
        asm!(
        "mov rax, 0x2000004", // system call 0x2000004 is write on macos
        "mov rdi, 1",         // file handle 1 is stdout
        "syscall",            // call kernel, syscall interrupt
        in("rsi") msg_ptr,    // address of string to output
        in("rdx") len,         // number of bytes
        out("rax") _, out("rdi") _, lateout("rsi") _, lateout("rdx") _
        );
    }
}

// ----------------------------------------------------------------------------
// macOS raw syscall when running newer M family CPU (ARM 64 architecture)
// ----------------------------------------------------------------------------

#[cfg(all(target_os = "macos", target_arch = "aarch64"))]
#[inline(never)]
fn syscall(message: String) {
    let ptr = message.as_ptr();
    let len = message.len();
    unsafe {
        asm!(
        // Moves the value `4` into register `x16`. This value represents the system call number for the `write` system call on macOS.
        "mov x16, 4",
        // Moves the value `1` into register `x0`. This value 1 represents the file descriptor for `stdout`.
        "mov x0, 1",
        // Executes a supervisor call (system call) with the number `0`, which triggers the system call specified in `x16`.
        "svc 0",
        // Passes the pointer to the message string as an input in register `x1`.
        in("x1") ptr,
        // Passes the length of the message string as an input in register `x2`.
        in("x2") len,
        // Specifies that `x16` is an output register, but the output value is not used.
        out("x16") _,
        // Specifies that `x0` is an output register, but the output value is not used.
        out("x0") _,
        // Specifies that `x1` is a late output register, but the output value is not used.
        lateout("x1") _,
        // Specifies that `x2` is a late output register, but the output value is not used.
        lateout("x2") _
        );
    }
}

// ----------------------------------------------------------------------------
// Windows raw syscall
// ----------------------------------------------------------------------------
#[cfg(target_os = "windows")]
#[inline(never)]
fn syscall(message: String) {
    panic!("We can't. Windows doesn't have a stable syscall ABI")
}
