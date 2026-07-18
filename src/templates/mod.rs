pub fn rv_toml(name: &str) -> String {
    format!(
        r#"[project]
name = "{name}"

[target]
arch = "rv64imac"
abi = "lp64"

# [sources]
# main = "main.S"
# c_files = ["helper.c"]

[toolchain]
cc = "riscv64-elf-gcc"
objdump = "riscv64-elf-objdump"
nm = "riscv64-elf-nm"
readelf = "riscv64-elf-readelf"
gdb = "riscv64-elf-gdb"

[build]
optimization = "0"
# static_link = false
# compiler_flags = ["-Wall"]
# assembler_flags = []
# linker_flags = []

# [link]
# driver = "ld"        # "ld" = bare metal (-nostdlib), "cc" = compiler driver (libc)
# libraries = []
# library_paths = []
# script = "linker.ld"

# [compile]
# generate_debug_symbols = false

[output]
directory = "build"
# binary = "{name}.elf"

[qemu]
mode = "user"
binary = "qemu-riscv64"
# args = ["-L", "/usr/riscv64-linux-gnu"]
"#
    )
}

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
