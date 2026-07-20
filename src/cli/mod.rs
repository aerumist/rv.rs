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
        /// Print commands as they execute
        #[arg(short, long)]
        verbose: bool,
    },
    /// Build and run in QEMU
    Run {
        /// Print commands as they execute
        #[arg(short, long)]
        verbose: bool,
    },
    /// Start QEMU with GDB attached
    Debug {
        /// Print commands as they execute
        #[arg(short, long)]
        verbose: bool,
    },
    /// Disassemble the ELF binary
    Disasm {
        /// Interleave source lines with disassembly
        #[arg(short, long)]
        source: bool,
    },
    /// Display symbols from the ELF binary
    Symbols,
    /// Display ELF sections
    Sections,
    /// Hex dump of the ELF binary
    Hex {
        /// Dump a specific section
        #[arg(short, long)]
        section: Option<String>,
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
            Command::Build { verbose } => commands::build::run(verbose),
            Command::Run { verbose } => commands::run::run(verbose),
            Command::Debug { verbose } => commands::debug::run(verbose),
            Command::Disasm { source } => commands::disasm::run(source),
            Command::Symbols => commands::symbols::run(),
            Command::Sections => commands::sections::run(),
            Command::Hex { section } => {
                commands::hex::run(section.as_deref())
            }
            Command::Clean => commands::clean::run(),
            Command::Watch => commands::watch::run(),
        }
    }
}
