use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use colored::Colorize;

use crate::compiler;
use crate::config::Config;

pub fn run(name: Option<&str>) -> Result<()> {
    let config = Config::load()?;
    let target = config.resolve_target(name);

    compiler::gcc::run(&config, Some(&target), false)?;
    let elf = config.elf_path(&target)?;

    println!("{:>12} {}", "Disasm".cyan().bold(), elf.display());

    let output = Command::new(&config.toolchain.objdump)
        .args(["-d", "-M", "no-aliases"])
        .arg(&elf)
        .stderr(Stdio::piped())
        .output()
        .with_context(|| {
            format!(
                "Failed to run objdump '{}'.\n\
                 Is the RISC-V toolchain installed?",
                config.toolchain.objdump
            )
        })?;

    if !output.status.success() {
        std::io::stderr().write_all(&output.stderr)?;
        anyhow::bail!("objdump failed.");
    }

    std::io::stdout().write_all(&output.stdout)?;
    Ok(())
}
