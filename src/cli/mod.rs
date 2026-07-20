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
        /// Specific source file (without extension)
        name: Option<String>,
        /// Print commands as they execute
        #[arg(short, long)]
        verbose: bool,
    },
    /// Build and run in QEMU
    Run {
        /// Specific source file (without extension)
        name: Option<String>,
        /// Print commands as they execute
        #[arg(short, long)]
        verbose: bool,
    },
    /// Start QEMU with GDB attached
    Debug {
        /// Specific source file (without extension)
        name: Option<String>,
        /// Print commands as they execute
        #[arg(short, long)]
        verbose: bool,
    },
    /// Disassemble the ELF binary
    Disasm {
        /// Specific source file (without extension)
        name: Option<String>,
        /// Interleave source lines with disassembly
        #[arg(short, long)]
        source: bool,
    },
    /// Display symbols from the ELF binary
    Symbols {
        /// Specific source file (without extension)
        name: Option<String>,
    },
    /// Display ELF sections
    Sections {
        /// Specific source file (without extension)
        name: Option<String>,
    },
    /// Hex dump of the ELF binary
    Hex {
        /// Specific source file (without extension)
        name: Option<String>,
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
            Command::Build { name, verbose } => commands::build::run(name.as_deref(), verbose),
            Command::Run { name, verbose } => commands::run::run(name.as_deref(), verbose),
            Command::Debug { name, verbose } => commands::debug::run(name.as_deref(), verbose),
            Command::Disasm { name, source } => {
                commands::disasm::run(name.as_deref(), source)
            }
            Command::Symbols { name } => commands::symbols::run(name.as_deref()),
            Command::Sections { name } => commands::sections::run(name.as_deref()),
            Command::Hex { name, section } => {
                commands::hex::run(name.as_deref(), section.as_deref())
            }
            Command::Clean => commands::clean::run(),
            Command::Watch => commands::watch::run(),
        }
    }
}
