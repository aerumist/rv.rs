# rv — RISC-V Assembly Development Tool

A Cargo-like CLI for writing, building, running, and debugging RISC-V assembly programs.

## Install

```bash
cargo install --path .
```

Or just use `cargo build --release` and put `target/release/rv` on your PATH.

## Prerequisites

- RISC-V GCC toolchain (`riscv64-elf-gcc` or similar)
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
| `rv build [name]` | Compile assembly to ELF |
| `rv run [name]` | Build and run in QEMU |
| `rv debug [name]` | Build, start QEMU with GDB server, attach GDB |
| `rv disasm [name]` | Disassemble the ELF with objdump |
| `rv symbols [name]` | List symbols with nm |
| `rv sections [name]` | Show ELF sections with readelf |
| `rv clean` | Remove the build directory |
| `rv watch` | Rebuild on source file changes |

All commands that accept `[name]` default to the project name from `rv.toml`.

## Configuration

Each project has an `rv.toml`:

```toml
[project]
name = "hello"

[target]
arch = "rv64imac"
abi = "lp64"

[compiler]
cc = "riscv64-elf-gcc"
objdump = "riscv64-elf-objdump"
nm = "riscv64-elf-nm"
readelf = "riscv64-elf-readelf"
gdb = "riscv64-elf-gdb"

[qemu]
user = "qemu-riscv64"
system = "qemu-system-riscv64"
mode = "user"  # or "system"

[paths]
source = "src"
build = "build"

[build]
opt_level = "0"
```

## Architecture

```
src/
├── main.rs          Entry point
├── cli/mod.rs       Clap-derived CLI definition
├── commands/        One module per command
├── compiler/gcc.rs  Compilation and linking
├── config/mod.rs    rv.toml deserialization
├── gdb/debug.rs     GDB/QEMU debug orchestration
├── qemu/run.rs      QEMU execution
└── templates/       Project scaffolding templates
```

Adding a new command: add a variant to `cli/mod.rs::Command`, a module in `commands/`, wire it in `Cli::run()`.

## Future

The architecture supports future additions like `rv flash`, `rv monitor`, `rv esp-build` for ESP32-C6 targets without structural changes — just new command modules and config sections.
