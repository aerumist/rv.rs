use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use colored::Colorize;

use crate::compiler;
use crate::config::Config;

pub fn run(source: bool) -> Result<()> {
    let config = Config::load()?;
    let target = config.resolve_target(None);

    compiler::gcc::run(&config, false)?;
    let elf = config.elf_path(&target)?;

    println!("{:>12} {}", "Disasm".cyan().bold(), elf.display());

    let mut args = vec!["-d".to_string(), "-M".to_string(), "no-aliases".to_string()];
    if source {
        args.push("-S".to_string());
    }

    let output = Command::new(&config.toolchain.objdump)
        .args(&args)
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
