use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use colored::Colorize;

use crate::compiler;
use crate::config::Config;

pub fn run(section: Option<&str>) -> Result<()> {
    let config = Config::load()?;
    let target = config.resolve_target(None);

    compiler::gcc::run(&config, false)?;
    let elf = config.elf_path(&target)?;

    println!("{:>12} {}", "Hex dump".cyan().bold(), elf.display());

    let mut cmd = Command::new(&config.toolchain.objdump);
    cmd.arg("-s");
    if let Some(sec) = section {
        cmd.arg(format!("--section={sec}"));
    }
    cmd.arg(&elf);

    let output = cmd
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
