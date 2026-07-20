# test.S — RISC-V assembly program

.section .rodata
msg:
    .ascii "Hello from test!\n"
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
