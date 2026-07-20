# Changelog

All notable changes to this project will be documented here.

## [Unreleased]

### Features

- Add hex dump command, source-interleaved disassembly
- Add hex command and update README with new commands
- Implement file hashing for source change detection in watch command

## [0.3.0] - 2026-07-20

### Documentation

- Add releasing instructions to CONTRIBUTING.md

### Miscellaneous

- Update dependency lockfile

## [0.2.0] - 2026-07-19

### Features

- Interactive project setup with compiler detection and templates
- Validate ISA/ABI, fix templates for Linux userspace, show exit status
- Add GitHub Actions CI workflow and CI status badge to README
- Replace automated post-commit changelog generation with a manual release script

### Styling

- Update README with enhanced aesthetic shields, light/dark mode support, and star history chart

### Miscellaneous

- Update 44 files across 7 directories
- Update 26 files across 3 directories
- Update README with new badges and improved markdown formatting
- Update shieldcn badge styles and repository URLs in README

## [0.1.0] - 2026-07-19

### Features

- Initial project scaffold with CLI, build, run, debug, disasm, symbols, sections, clean, and watch commands
- Enhance build system to support C files and verbose output
- Update 17 files across 7 directories

### Build

- Add commit-msg hook enforcing conventional commits
