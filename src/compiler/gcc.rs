use std::io::Write;
use std::process::{Command, Stdio};
use std::time::Instant;

use anyhow::{Context, Result, bail};
use colored::Colorize;

use crate::config::Config;

pub fn run(config: &Config, name: Option<&str>) -> Result<()> {
    let start = Instant::now();
    let target = config.resolve_target(name);
    let build_dir = config.build_dir()?;
    std::fs::create_dir_all(&build_dir)?;

    let sources = if name.is_some() {
        vec![config.find_source(&target)?]
    } else {
        config.all_sources()?
    };

    if sources.is_empty() {
        bail!("No assembly source files found in '{}'.", config.paths.source);
    }

    let mut objects = Vec::new();

    for src in &sources {
        let stem = src.file_stem().unwrap().to_string_lossy();
        let obj = build_dir.join(format!("{stem}.o"));
        println!(
            "{:>12} {}",
            "Compiling".green().bold(),
            src.file_name().unwrap().to_string_lossy()
        );

        let output = Command::new(&config.compiler.cc)
            .args([
                "-c",
                &format!("-march={}", config.target.arch),
                &format!("-mabi={}", config.target.abi),
                &format!("-O{}", config.build.opt_level),
                "-o",
            ])
            .arg(&obj)
            .arg(src)
            .stderr(Stdio::piped())
            .stdout(Stdio::piped())
            .output()
            .with_context(|| {
                format!(
                    "Failed to run compiler '{}'.\n\
                     Is the RISC-V toolchain installed?",
                    config.compiler.cc
                )
            })?;

        if !output.status.success() {
            std::io::stderr().write_all(&output.stderr)?;
            bail!("Compilation failed for {}", src.display());
        }

        objects.push(obj);
    }

    let elf = config.elf_path(&target)?;
    println!(
        "{:>12} {}",
        "Linking".green().bold(),
        elf.file_name().unwrap().to_string_lossy()
    );

    let output = Command::new(&config.compiler.cc)
        .args([
            "-nostdlib",
            &format!("-march={}", config.target.arch),
            &format!("-mabi={}", config.target.abi),
            "-o",
        ])
        .arg(&elf)
        .args(&objects)
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()
        .with_context(|| format!("Failed to run linker '{}'.", config.compiler.cc))?;

    if !output.status.success() {
        std::io::stderr().write_all(&output.stderr)?;
        bail!("Linking failed.");
    }

    let elapsed = start.elapsed();
    println!(
        "{:>12} in {:.2}s",
        "Finished".green().bold(),
        elapsed.as_secs_f64()
    );

    Ok(())
}
