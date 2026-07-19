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
# Demonstrates calling C functions from assembly running in QEMU user mode.

.section .text
.global main
.extern print_banner, compute_fibonacci, print_result

main:
    # Prologue: save ra and s0
    addi sp, sp, -16
    sd   ra, 8(sp)
    sd   s0, 0(sp)

    # Print the welcome banner
    call print_banner

    # Compute fibonacci(10), save result in s0
    li   a0, 10
    call compute_fibonacci
    mv   s0, a0

    # Print the result
    mv   a0, s0
    call print_result

    # Return fibonacci result as exit code
    mv   a0, s0

    # Epilogue: restore ra and s0
    ld   ra, 8(sp)
    ld   s0, 0(sp)
    addi sp, sp, 16
    ret
"#
    )
}

pub fn starter_c_qemu() -> &'static str {
    r#"// helper.c — C helper functions called from assembly

#include <stdio.h>
#include <unistd.h>

void print_banner(void) {
    const char *msg = "=== RISC-V QEMU Mixed ASM+C Project ===\n";
    write(1, msg, 41);
}

long compute_fibonacci(int n) {
    long a = 0, b = 1;
    for (int i = 0; i < n; i++) {
        long tmp = a + b;
        a = b;
        b = tmp;
    }
    return a;
}

void print_result(long value) {
    printf("Fibonacci(10) = %ld\n", value);
}
"#
}
