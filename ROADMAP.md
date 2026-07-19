# Roadmap

This roadmap reflects current plans for rv. Priorities may shift based on community feedback.

## v0.1 — Foundation

- [x] Project scaffolding (`rv new`)
- [x] Assembly compilation and linking
- [x] QEMU user-mode execution
- [x] Basic `rv.toml` configuration
- [x] Linux RISC-V userspace support

## v0.2 — Mixed Language Support

- [x] C source file compilation
- [x] Verbose mode (`--verbose`)
- [ ] Improved error messages with source context
- [ ] Config validation with actionable diagnostics

## v0.3 — Inspection and Debugging

- [x] GDB integration (`rv debug`)
- [x] Disassembly (`rv disasm`)
- [x] Symbol listing (`rv symbols`)
- [x] Section listing (`rv sections`)
- [ ] Hex dump command
- [ ] Source-interleaved disassembly

## v0.4 — Bare Metal

- [ ] Linker script support
- [ ] QEMU system-mode execution
- [ ] Bare-metal project templates
- [ ] Memory map visualization

## v0.5 — Embedded Targets

- [ ] ESP32-C6 support (RV32IMAC)
- [ ] Flash/upload integration
- [ ] Target-specific templates
- [ ] OpenOCD integration

## Future

- [ ] Plugin system for custom commands
- [ ] Multiple architecture support beyond RISC-V
- [ ] Project templates repository
- [ ] LLVM/Clang toolchain support
- [ ] Remote debugging over network
- [ ] Integration test suite
- [ ] Package registry for shared linker scripts and templates

---

Want to help? Pick an unchecked item and open an issue to discuss your approach before starting work. See [CONTRIBUTING.md](CONTRIBUTING.md).
