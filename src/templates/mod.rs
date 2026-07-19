pub fn starter_asm(name: &str) -> String {
    format!(
        r#"# {name}.S — RISC-V assembly program

.section .rodata
msg:
    .ascii "Hello from {name}!\n"
    .equ msg_len, . - msg

.section .text
.global _start

_start:
    # write(stdout, msg, msg_len)
    li a0, 1
    la a1, msg
    li a2, msg_len
    li a7, 64
    ecall

    # exit(0)
    li a0, 0
    li a7, 93
    ecall
"#
    )
}

pub fn gitignore() -> &'static str {
    "build/\n"
}

pub fn starter_asm_qemu(name: &str) -> String {
    format!(
        r#"# {name} — RISC-V assembly + C mixed project
# Demonstrates calling a C function from assembly running in QEMU user mode.

.section .text
.global _start
.extern print_banner, compute_fibonacci

_start:
    # Call C function to print a welcome banner
    call print_banner

    # Compute fibonacci(10) in C, result in a0
    li a0, 10
    call compute_fibonacci

    # Exit with fibonacci result as the exit code
    # (rv run will show: "Process exited with code 55")
    li a7, 93
    ecall
"#
    )
}

pub fn starter_c_qemu() -> &'static str {
    r#"// helper.c — C helper functions called from assembly
// Compiled alongside the assembly entry point and linked via gcc driver.

#include <unistd.h>

static long fib(int n) {
    long a = 0, b = 1;
    for (int i = 0; i < n; i++) {
        long tmp = a + b;
        a = b;
        b = tmp;
    }
    return a;
}

void print_banner(void) {
    const char *msg = "=== RISC-V QEMU Mixed ASM+C Project ===\n";
    write(1, msg, 41);
}

long compute_fibonacci(int n) {
    return fib(n);
}
"#
}
