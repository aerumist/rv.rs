use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use colored::Colorize;

use crate::config::{Config, QemuMode};

pub fn run(config: &Config, elf: &std::path::Path) -> Result<()> {
    println!(
        "{:>12} {}",
        "Debugging".green().bold(),
        elf.display()
    );

    // Start QEMU waiting for GDB
    let (qemu_cmd, qemu_args) = match config.qemu.mode {
        QemuMode::User => (
            &config.qemu.user,
            vec!["-g".into(), "1234".into(), elf.to_string_lossy().to_string()],
        ),
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
                "-s".into(),
                "-S".into(),
            ],
        ),
    };

    println!(
        "{:>12} QEMU on :1234 (waiting for GDB)",
        "Starting".cyan().bold()
    );

    let mut qemu = Command::new(qemu_cmd)
        .args(&qemu_args)
        .stdin(Stdio::null())
        .stdout(Stdio::piped())
        .stderr(Stdio::piped())
        .spawn()
        .with_context(|| format!("Failed to start QEMU '{qemu_cmd}'."))?;

    // Give QEMU a moment to open the port
    std::thread::sleep(std::time::Duration::from_millis(200));

    // Write a temporary GDB script
    let gdb_script = format!(
        "target remote :1234\n\
         file {elf}\n\
         layout asm\n\
         layout regs\n",
        elf = elf.display()
    );

    let gdb_script_path = config.build_dir()?.join(".gdbinit");
    std::fs::write(&gdb_script_path, &gdb_script)?;

    println!(
        "{:>12} GDB → :1234",
        "Connecting".cyan().bold()
    );

    let _gdb_status = Command::new(&config.compiler.gdb)
        .args(["-x", &gdb_script_path.to_string_lossy()])
        .stdin(Stdio::inherit())
        .stdout(Stdio::inherit())
        .stderr(Stdio::inherit())
        .status()
        .with_context(|| {
            format!(
                "Failed to run GDB '{}'.\n\
                 Install with: pacman -S riscv64-elf-gdb",
                config.compiler.gdb
            )
        })?;

    // Clean up QEMU
    let _ = qemu.kill();
    let _ = qemu.wait();
    let _ = std::fs::remove_file(&gdb_script_path);

    Ok(())
}
