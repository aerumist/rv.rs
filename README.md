# rv — RISC-V Development Tool

A Cargo-like CLI for writing, building, running, and debugging RISC-V assembly and C programs.

## Install

```bash
cargo install --path .
```

Or just use `cargo build --release` and put `target/release/rv` on your PATH.

## Prerequisites

- RISC-V GCC toolchain (`riscv64-elf-gcc`, `riscv64-linux-gnu-gcc`, or similar)
- QEMU (`qemu-riscv64` for user-mode, `qemu-system-riscv64` for system)
- GDB (optional, for `rv debug`)

On Arch Linux:
```bash
pacman -S riscv64-elf-gcc riscv64-elf-binutils qemu-user qemu-system-riscv
```

## Quick Start

```bash
rv new hello
cd hello
rv build
rv run
```

## Commands

| Command | Description |
|---------|-------------|
| `rv new <name>` | Create a new project with starter assembly |
| `rv build [name]` | Compile sources to ELF |
| `rv run [name]` | Build and run in QEMU |
| `rv debug [name]` | Build, start QEMU with GDB server, attach GDB |
| `rv disasm [name]` | Disassemble the ELF with objdump |
| `rv symbols [name]` | List symbols with nm |
| `rv sections [name]` | Show ELF sections with readelf |
| `rv clean` | Remove the build directory |
| `rv watch` | Rebuild on source file changes |

All commands that accept `[name]` default to the project name from `rv.toml`.

`rv build`, `rv run`, and `rv debug` accept `--verbose` / `-v` to print the exact commands being executed.

## Configuration

Each project has an `rv.toml`. Every section except `[project]` is optional with sensible defaults.

```toml
[project]
name = "hello"

[target]
arch = "rv64gc"
abi = "lp64d"

[sources]
main = "boot.S"              # override default main assembly file
c_files = ["helper.c"]       # C files to compile and link

[toolchain]
cc = "riscv64-linux-gnu-gcc"
objdump = "riscv64-linux-gnu-objdump"
nm = "riscv64-linux-gnu-nm"
readelf = "riscv64-linux-gnu-readelf"
gdb = "riscv64-linux-gnu-gdb"

[build]
optimization = "0"
static = true
compiler_flags = ["-Wall"]   # flags for C compilation
assembler_flags = []         # flags for assembly compilation
linker_flags = []            # extra flags passed to linker

[link]
driver = "cc"                # "ld" = bare metal (-nostdlib), "cc" = compiler driver (libc)
libraries = ["m"]
library_paths = ["/usr/riscv64-linux-gnu/lib"]
script = "linker.ld"         # linker script (bare metal)

[compile]
generate_debug_symbols = true

[output]
directory = "build"
binary = "hello.elf"         # override output filename

[qemu]
mode = "user"
binary = "qemu-riscv64"
args = ["-L", "/usr/riscv64-linux-gnu"]
```

### Link drivers

- `driver = "ld"` (default) — bare metal. Passes `-nostdlib`, you provide `_start`.
- `driver = "cc"` — uses the compiler driver. Links crt startup, libc, libgcc automatically. Use with `main()` in C or assembly.

### Object file naming

Object files are named `{filename}.o` (e.g. `main.s.o`, `helper.c.o`) to avoid collisions when assembly and C files share a stem.

### Verbose mode

```
$ rv build --verbose

  riscv64-linux-gnu-gcc \
    -c \
    -march=rv64gc \
    -mabi=lp64d \
    -O0 \
    -Wall \
    src/helper.c \
    -o \
    build/helper.c.o
```

## Target Examples

### Bare metal (default)

```toml
[target]
arch = "rv64imac"
abi = "lp64"

[toolchain]
cc = "riscv64-elf-gcc"
```

### Linux user-mode

```toml
[target]
arch = "rv64gc"
abi = "lp64d"

[toolchain]
cc = "riscv64-linux-gnu-gcc"

[build]
static = true

[link]
driver = "cc"

[qemu]
binary = "qemu-riscv64"
args = ["-L", "/usr/riscv64-linux-gnu"]
```

### ESP32-C6 (future)

```toml
[target]
arch = "rv32imac"
abi = "ilp32"

[toolchain]
cc = "riscv32-esp-elf-gcc"

[link]
script = "esp32c6.ld"
```

No Rust code changes required — just a different `rv.toml`.

## Architecture

```
src/
├── main.rs          Entry point
├── cli/mod.rs       Clap-derived CLI definition
├── commands/        One module per command
├── compiler/gcc.rs  Compilation and linking pipeline
├── config/mod.rs    rv.toml deserialization
├── gdb/debug.rs     GDB/QEMU debug orchestration
├── qemu/run.rs      QEMU execution
└── templates/       Project scaffolding templates
```

Adding a new command: add a variant to `cli/mod.rs::Command`, a module in `commands/`, wire it in `Cli::run()`.
