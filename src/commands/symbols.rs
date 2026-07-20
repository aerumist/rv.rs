use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use colored::Colorize;

use crate::compiler;
use crate::config::Config;

pub fn run() -> Result<()> {
    let config = Config::load()?;
    let target = config.resolve_target(None);
    compiler::gcc::run(&config, false)?;
    let elf = config.elf_path(&target)?;

    println!("{:>12} {}", "Symbols".cyan().bold(), elf.display());

    let output = Command::new(&config.toolchain.nm)
        .arg(&elf)
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("Failed to run nm '{}'.", config.toolchain.nm))?;

    if !output.status.success() {
        std::io::stderr().write_all(&output.stderr)?;
        anyhow::bail!("nm failed.");
    }

    std::io::stdout().write_all(&output.stdout)?;
    Ok(())
}
