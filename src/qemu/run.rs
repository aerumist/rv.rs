use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use colored::Colorize;

use crate::config::{Config, QemuMode};

pub fn run(config: &Config, elf: &std::path::Path) -> Result<()> {
    let (cmd, args) = match config.qemu.mode {
        QemuMode::User => (&config.qemu.user, vec![elf.to_string_lossy().to_string()]),
        QemuMode::System => (
            &config.qemu.system,
            vec![
                "-nographic".into(),
                "-machine".into(),
                "virt".into(),
                "-bios".into(),
                "none".into(),
                "-kernel".into(),
                elf.to_string_lossy().to_string(),
            ],
        ),
    };

    println!("{:>12} {}", "Running".green().bold(), elf.display());

    let status = Command::new(cmd)
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| {
            format!(
                "Failed to run QEMU '{cmd}'.\n\
                 Is QEMU installed? Try: pacman -S qemu-user"
            )
        })?;

    if !status.success() {
        let code = status.code().unwrap_or(-1);
        // QEMU user mode returns the guest exit code
        std::process::exit(code);
    }

    Ok(())
}
