mod cli;
mod commands;
mod compiler;
mod config;
mod gdb;
mod qemu;
mod templates;

use anyhow::Result;
use cli::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse_args();
    cli.run()
}
