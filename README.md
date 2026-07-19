# rv

**A Cargo-like CLI for RISC-V assembly and C development.**

<p align="center">
  <a href="LICENSE"><img src="https://shieldcn.dev/github/license/aerumist/rv.svg" alt="License: MIT"></a>
  <a href="https://www.rust-lang.org/"><img src="https://shieldcn.dev/badge/rust-2024-orange.svg?logo=rust" alt="Rust 2024"></a>
  <img src="https://shieldcn.dev/badge/C-blue.svg?logo=c" alt="C">
  <img src="https://shieldcn.dev/badge/RISC--V-black.svg?logo=riscv" alt="RISC-V">
  <a href="https://www.qemu.org/"><img src="https://shieldcn.dev/badge/QEMU-orange.svg?logo=qemu" alt="QEMU"></a>
  <a href="https://www.sourceware.org/gdb/"><img src="https://shieldcn.dev/badge/GDB-purple.svg?logo=gnu" alt="GDB"></a>
</p>

---

## Why rv?

Writing RISC-V assembly means juggling cross-compilers, linker scripts, QEMU flags, and GDB configurations. `rv` wraps all of that behind a single command-line tool with a familiar Cargo-like interface.

One config file. One command to build. One command to run.

## Features

- Cargo-like workflow (`rv new`, `rv build`, `rv run`, `rv debug`)
- Configuration-driven builds via `rv.toml`
- Mixed C and assembly projects
- QEMU integration (user-mode and system-mode)
- GDB debugging with automatic stub setup
- ELF inspection (disassembly, symbols, sections)
- File watching with auto-rebuild
- Verbose mode showing exact toolchain commands
- Extensible target system (bare metal, Linux user-mode, ESP32-C6)
- No Rust code changes needed for new targets

## Installation

### From source

```bash
cargo install --path .
```

Or build and add to PATH manually:

```bash
cargo build --release
# Binary is at target/release/rv
```

### Prerequisites

- RISC-V GCC toolchain (`riscv64-elf-gcc` or `riscv64-linux-gnu-gcc`)
- QEMU (`qemu-riscv64` for user-mode, `qemu-system-riscv64` for system-mode)
- GDB (`riscv64-elf-gdb`, optional, for `rv debug`)

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

This creates a project with a hello-world assembly program that uses Linux syscalls to print and exit.

### Generated `rv.toml`

```toml
[project]
name = "hello"

[target]
arch = "rv64imac"
abi = "lp64"

[toolchain]
cc = "riscv64-elf-gcc"

[output]
directory = "build"

[qemu]
mode = "user"
binary = "qemu-riscv64"
```

## Commands

| Command              | Description                                   |
| -------------------- | --------------------------------------------- |
| `rv new <name>`      | Create a new project with starter assembly    |
| `rv build [name]`    | Compile sources to ELF                        |
| `rv run [name]`      | Build and run in QEMU                         |
| `rv debug [name]`    | Build, start QEMU with GDB server, attach GDB |
| `rv disasm [name]`   | Disassemble the ELF with objdump              |
| `rv symbols [name]`  | List symbols with nm                          |
| `rv sections [name]` | Show ELF sections with readelf                |
| `rv clean`           | Remove the build directory                    |
| `rv watch`           | Rebuild on source file changes                |

Commands accepting `[name]` default to the project name from `rv.toml`.

Use `--verbose` / `-v` with `build`, `run`, or `debug` to print the exact commands being executed.

## Configuration

Each project is configured via `rv.toml`. Only `[project]` is required; everything else has sensible defaults.

```toml
[project]
name = "hello"

[target]
arch = "rv64gc"          # RISC-V ISA string
abi = "lp64d"            # ABI (lp64, lp64d, ilp32, etc.)

[sources]
main = "boot.S"          # override default main assembly file
c_files = ["helper.c"]   # C files to compile and link

[toolchain]
cc = "riscv64-linux-gnu-gcc"
objdump = "riscv64-linux-gnu-objdump"
nm = "riscv64-linux-gnu-nm"
readelf = "riscv64-linux-gnu-readelf"
gdb = "riscv64-linux-gnu-gdb"

[build]
optimization = "0"
static = true
compiler_flags = ["-Wall"]
assembler_flags = []
linker_flags = []

[link]
driver = "cc"            # "ld" = bare metal, "cc" = compiler driver (libc)
libraries = ["m"]
library_paths = ["/usr/riscv64-linux-gnu/lib"]
script = "linker.ld"     # linker script (bare metal)

[compile]
generate_debug_symbols = true

[output]
directory = "build"
binary = "hello.elf"

[qemu]
mode = "user"
binary = "qemu-riscv64"
args = ["-L", "/usr/riscv64-linux-gnu"]
```

### Link drivers

| Driver         | Use case            | Behavior                                      |
| -------------- | ------------------- | --------------------------------------------- |
| `ld` (default) | Bare metal          | Passes `-nostdlib`, you provide `_start`      |
| `cc`           | User-mode with libc | Links crt startup, libc, libgcc automatically |

## Supported Targets

rv supports any RISC-V target that GCC can compile for. Configuration examples:

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

### ESP32-C6 (planned)

```toml
[target]
arch = "rv32imac"
abi = "ilp32"

[toolchain]
cc = "riscv32-esp-elf-gcc"

[link]
script = "esp32c6.ld"
```

No source code changes required for new targets. Just write a different `rv.toml`.

## Documentation

- [Contributing Guide](CONTRIBUTING.md)
- [Architecture](ARCHITECTURE.md)
- [Roadmap](ROADMAP.md)
- [Changelog](CHANGELOG.md)

## Roadmap

See [ROADMAP.md](ROADMAP.md) for the full plan. Highlights:

- Bare-metal target support with linker scripts
- ESP32-C6 support
- Project templates
- Plugin system

## Contributing

Contributions are welcome. See [CONTRIBUTING.md](CONTRIBUTING.md) for guidelines.

## License

[MIT](LICENSE) &copy; 2026 Tahsin Ahmed
