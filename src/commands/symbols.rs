use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use colored::Colorize;

use crate::compiler;
use crate::config::Config;

pub fn run(name: Option<&str>) -> Result<()> {
    let config = Config::load()?;
    let target = config.resolve_target(name);
    compiler::gcc::run(&config, Some(&target))?;
    let elf = config.elf_path(&target)?;

    println!("{:>12} {}", "Symbols".cyan().bold(), elf.display());

    let output = Command::new(&config.compiler.nm)
        .arg(&elf)
        .stderr(Stdio::piped())
        .output()
        .with_context(|| {
            format!("Failed to run nm '{}'.", config.compiler.nm)
        })?;

    if !output.status.success() {
        std::io::stderr().write_all(&output.stderr)?;
        anyhow::bail!("nm failed.");
    }

    std::io::stdout().write_all(&output.stdout)?;
    Ok(())
}
