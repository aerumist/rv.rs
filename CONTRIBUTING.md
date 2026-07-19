# Contributing to rv

Welcome, and thank you for considering contributing. Whether you're fixing a bug, proposing a feature, or improving documentation, your help is appreciated.

## Project Philosophy

rv is a configuration-driven CLI that wraps the RISC-V GCC toolchain and QEMU into a seamless workflow. These principles guide development:

- **Configuration over code** — Target support comes from `rv.toml`, not from special-casing in Rust. The tool should adapt through its config layer.
- **Readable over clever** — Straightforward code that a newcomer can follow beats terse or overly abstract implementations.
- **Modular design** — Each subcommand lives in its own module. Shared concerns (compilation, QEMU invocation, GDB) are isolated behind clear interfaces.
- **Idiomatic Rust** — Standard Rust conventions, strong types, ecosystem libraries where appropriate.

## Development Setup

### Prerequisites

| Tool | Purpose |
|------|---------|
| [Rust](https://rustup.rs/) (stable) | Building rv itself |
| `riscv64-elf-gcc` + binutils | Cross-compilation toolchain |
| `qemu-riscv64` | Running RISC-V binaries |
| `riscv64-elf-gdb` | Debugging (optional) |
| [git-cliff](https://git-cliff.org/) | Changelog generation (optional) |

#### Arch Linux

```bash
pacman -S riscv64-elf-gcc riscv64-elf-binutils riscv64-elf-gdb qemu-user qemu-system-riscv
cargo install git-cliff
```

#### Other distributions

Install the RISC-V GCC toolchain from your package manager or build from source via [riscv-gnu-toolchain](https://github.com/riscv-collab/riscv-gnu-toolchain). QEMU packages are widely available.

### Building

```bash
cargo build              # dev build
cargo build --release    # optimized release (stripped, LTO)
cargo install --path .   # install to ~/.cargo/bin
```

### Running Tests

A formal test suite does not exist yet. Verify changes by building the project and manually testing affected subcommands against a sample project:

```bash
rv new test_project          # interactive setup
cd test_project
rv build --verbose
rv run
```

Or with a template to skip most prompts:

```bash
rv new test_project --template qemu
```

If you'd like to contribute tests, that is very welcome.

### Formatting

```bash
cargo fmt --all
```

All code must be formatted before submission.

### Linting

```bash
cargo clippy --all-targets -- -D warnings
```

Address all warnings. No `#[allow(...)]` without justification.

## Commits

This project uses [Conventional Commits](https://www.conventionalcommits.org/) and enforces them via a `commit-msg` git hook. Set it up after cloning:

```bash
git config core.hooksPath .githooks
```

```
feat: add verbose flag to build command
fix: resolve linker script path resolution
docs: update configuration reference
refactor: extract source discovery into helper
```

Rules:

- Imperative mood in the subject line
- One logical change per commit
- Keep subject under 72 characters
- Body explains *why*, not *what*

The changelog is generated from commit messages via [git-cliff](https://git-cliff.org/), so well-structured commits directly improve the project history.

## Branch Naming

Use a category prefix:

- `feat/description` — new features
- `fix/description` — bug fixes
- `docs/description` — documentation
- `refactor/description` — restructuring without behavior changes

## Pull Requests

- One feature or fix per PR
- Clear description of what and why
- Reference related issues
- Ensure `cargo build`, `cargo fmt --check`, and `cargo clippy` pass
- Be open to feedback and iteration

## Reporting Bugs

Include:

- What you were trying to do
- What happened instead
- Steps to reproduce
- Environment (OS, Rust version, toolchain versions)
- Relevant `rv.toml` configuration

## Feature Requests

When proposing a feature:

- Describe the use case and the problem it solves
- Explain how it fits with the configuration-driven philosophy
- Consider whether it belongs in rv itself or in the user's `rv.toml`

## Large Changes

If you're considering a significant change (new subcommand, architectural refactor, config format change), please open an issue to discuss it first. This avoids duplicated effort and ensures alignment with the project direction.

---

Thank you for helping make rv better.
