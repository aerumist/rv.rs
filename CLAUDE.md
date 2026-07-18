# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Build Commands

```bash
cargo build              # dev build
cargo build --release    # optimized release (stripped, LTO)
cargo install --path .   # install `rv` binary to ~/.cargo/bin
```

No test suite exists yet. No linter configuration beyond default `rustc` warnings.

## What This Is

`rv` is a Cargo-like CLI that wraps the RISC-V GCC cross-compilation toolchain (`riscv64-elf-*`) and QEMU to provide a single-command workflow for writing, building, running, and debugging RISC-V assembly programs.

## Architecture

The binary is structured as a command dispatcher:

- `src/main.rs` — entry point, delegates to `cli::Cli`
- `src/cli/mod.rs` — clap derive-based CLI definition; each `Command` variant maps to a module in `commands/`
- `src/commands/` — one file per subcommand (`build`, `run`, `debug`, `disasm`, `symbols`, `sections`, `clean`, `watch`, `new`)
- `src/config/mod.rs` — deserializes `rv.toml` (serde + toml), provides path resolution and source discovery
- `src/compiler/gcc.rs` — assembles `.S`/`.s`/`.asm` files and links ELF via `riscv64-elf-gcc`
- `src/qemu/run.rs` — executes ELF in QEMU (user or system mode)
- `src/gdb/debug.rs` — spawns QEMU with GDB stub, writes a temp `.gdbinit`, launches GDB
- `src/templates/mod.rs` — string templates for `rv new` scaffolding (`rv.toml`, starter `.S`, `.gitignore`)

**Control flow:** CLI parses args → loads `rv.toml` from cwd (walking up) → dispatches to command module → command calls into compiler/qemu/gdb as needed.

**Adding a command:** add an enum variant in `cli/mod.rs::Command`, create `src/commands/<name>.rs`, wire it in `commands/mod.rs` and `Cli::run()`.

## Runtime Dependencies

The tool shells out to external binaries. These must be on PATH:
- `riscv64-elf-gcc`, `riscv64-elf-objdump`, `riscv64-elf-nm`, `riscv64-elf-readelf`, `riscv64-elf-gdb`
- `qemu-riscv64` (user mode) or `qemu-system-riscv64` (system mode)

All tool names are configurable per-project in `rv.toml`.

## Project Layout (for rv-managed assembly projects)

```
<project>/
├── rv.toml       # project config
├── src/          # assembly sources (.S, .s, .asm)
└── build/        # compiled .o and .elf outputs
```
