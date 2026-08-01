use anyhow::Result;
use clap::{Parser, Subcommand};

use crate::commands;

#[derive(Parser)]
#[command(name = "rv", about = "RISC-V assembly development tool")]
pub struct Cli {
    #[command(subcommand)]
    command: Command,
}

#[derive(Subcommand)]
enum Command {
    /// Create a new RISC-V assembly project
    New {
        /// Project name
        name: String,
        /// Project template: default, qemu-asm, qemu-mixed
        #[arg(short, long, default_value = "default")]
        template: String,
    },
    /// Compile source files
    Build {
        /// Build a standalone file instead of the rv.toml project
        file: Option<String>,
        /// Print commands as they execute
        #[arg(short, long)]
        verbose: bool,
    },
    /// Build and run in QEMU
    Run {
        /// Run a standalone file instead of the rv.toml project
        file: Option<String>,
        /// Print commands as they execute
        #[arg(short, long)]
        verbose: bool,
    },
    /// Start QEMU with GDB attached
    Debug {
        /// Debug a standalone file instead of the rv.toml project
        file: Option<String>,
        /// Print commands as they execute
        #[arg(short, long)]
        verbose: bool,
    },
    /// Disassemble the ELF binary
    Disasm {
        /// Disassemble a standalone file instead of the rv.toml project
        file: Option<String>,
        /// Interleave source lines with disassembly
        #[arg(short, long)]
        source: bool,
    },
    /// Display symbols from the ELF binary
    Symbols {
        /// Inspect a standalone file instead of the rv.toml project
        file: Option<String>,
    },
    /// Display ELF sections
    Sections {
        /// Inspect a standalone file instead of the rv.toml project
        file: Option<String>,
    },
    /// Hex dump of the ELF binary
    Hex {
        /// Inspect a standalone file instead of the rv.toml project
        file: Option<String>,
        /// Dump a specific section
        #[arg(short, long)]
        section: Option<String>,
    },
    /// Visualize ELF memory map
    Memmap {
        /// Inspect a standalone file instead of the rv.toml project
        file: Option<String>,
        /// Print commands as they execute
        #[arg(short, long)]
        verbose: bool,
    },
    /// Remove the build directory
    Clean,
    /// Watch source files and rebuild on changes
    Watch,
}

impl Cli {
    pub fn parse_args() -> Self {
        Self::parse()
    }

    pub fn run(self) -> Result<()> {
        match self.command {
            Command::New { name, template } => commands::new::run(&name, &template),
            Command::Build { file, verbose } => commands::build::run(file.as_deref(), verbose),
            Command::Run { file, verbose } => commands::run::run(file.as_deref(), verbose),
            Command::Debug { file, verbose } => commands::debug::run(file.as_deref(), verbose),
            Command::Disasm { file, source } => commands::disasm::run(file.as_deref(), source),
            Command::Symbols { file } => commands::symbols::run(file.as_deref()),
            Command::Sections { file } => commands::sections::run(file.as_deref()),
            Command::Hex { file, section } => {
                commands::hex::run(file.as_deref(), section.as_deref())
            }
            Command::Memmap { file, verbose } => commands::memmap::run(file.as_deref(), verbose),
            Command::Clean => commands::clean::run(),
            Command::Watch => commands::watch::run(),
        }
    }
}
