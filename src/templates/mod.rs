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

pub fn starter_asm_bare(name: &str) -> String {
    format!(
        r#"# {name}.S — Bare-metal RISC-V startup for QEMU virt machine
# No OS, no libc. Runs in QEMU system mode (-machine virt).

.equ UART0, 0x10000000

.section .text.init
.global _start

_start:
    # Set up stack pointer (top of 128MB RAM)
    li t0, 0x88000000
    mv sp, t0

    # Clear BSS
    la t0, __bss_start
    la t1, __bss_end
1:  bge t0, t1, 2f
    sd zero, 0(t0)
    addi t0, t0, 8
    j 1b
2:
    # Print "Hello from {name}!" to UART0
    la a0, msg
    jal ra, uart_puts

    # Halt: write to QEMU finisher device to exit cleanly
    li t0, 0x100000
    li t1, 0x5555
    sw t1, 0(t0)

    # Fallback halt loop
3:  wfi
    j 3b

uart_puts:
    li t2, UART0
4:  lbu t3, 0(a0)
    beqz t3, 5f
    sw t3, 0(t2)
    addi a0, a0, 1
    j 4b
5:  ret

.section .rodata
msg:
    .ascii "Hello from {name}!\n"
    .equ msg_len, . - msg
"#
    )
}

pub fn linker_script_virt() -> &'static str {
    r#"/* Linker script for QEMU virt machine — 128MB RAM at 0x80000000 */

ENTRY(_start)

MEMORY
{
    RAM (rwx) : ORIGIN = 0x80000000, LENGTH = 128M
}

SECTIONS
{
    .text : {
        *(.text.init)
        *(.text .text.*)
    } > RAM

    .rodata : {
        *(.rodata .rodata.*)
    } > RAM

    .data : {
        *(.data .data.*)
    } > RAM

    .bss : {
        __bss_start = .;
        *(.bss .bss.*)
        *(COMMON)
        __bss_end = .;
    } > RAM

    /DISCARD/ : {
        *(.comment)
        *(.eh_frame)
    }
}
"#
}
