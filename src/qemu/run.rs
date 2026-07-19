use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use colored::Colorize;

use crate::config::{Config, QemuMode};

pub fn run(config: &Config, elf: &std::path::Path) -> Result<()> {
    let mut args: Vec<String> = config.qemu.args.clone();

    match config.qemu.mode {
        QemuMode::User => {
            args.push(elf.to_string_lossy().to_string());
        }
        QemuMode::System => {
            if args.is_empty() {
                args.extend([
                    "-nographic".into(),
                    "-machine".into(),
                    "virt".into(),
                    "-bios".into(),
                    "none".into(),
                    "-kernel".into(),
                    elf.to_string_lossy().to_string(),
                ]);
            } else {
                args.push(elf.to_string_lossy().to_string());
            }
        }
    }

    println!("{:>12} {}", "Running".green().bold(), elf.display());

    let status = Command::new(&config.qemu.binary)
        .args(&args)
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| {
            format!(
                "Failed to run QEMU '{}'.\n\
                 Is QEMU installed?",
                config.qemu.binary
            )
        })?;

    print_exit_status(&status);

    if !status.success() {
        let code = status.code().unwrap_or(1);
        std::process::exit(code);
    }

    Ok(())
}

fn print_exit_status(status: &std::process::ExitStatus) {
    #[cfg(unix)]
    {
        use std::os::unix::process::ExitStatusExt;
        if let Some(sig) = status.signal() {
            println!(
                "\n{}",
                format!("Process terminated by signal {sig}").yellow()
            );
            return;
        }
    }
    let code = status.code().unwrap_or(0);
    let msg = format!("Process exited with code {code}");
    if code == 0 {
        println!("\n{}", msg.dimmed());
    } else {
        println!("\n{}", msg.yellow());
    }
}
