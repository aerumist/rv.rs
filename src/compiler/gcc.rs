use std::io::Write;
use std::path::{Path, PathBuf};
use std::process::{Command, Stdio};
use std::time::Instant;

use anyhow::{Context, Result, bail};
use colored::Colorize;

use crate::config::{Config, LinkDriver};

pub fn run(config: &Config, name: Option<&str>, verbose: bool) -> Result<()> {
    let start = Instant::now();
    let target = config.resolve_target(name);
    let build_dir = config.build_dir()?;
    std::fs::create_dir_all(&build_dir)?;

    let mut sources = if name.is_some() || config.sources.main.is_some() {
        vec![config.find_source(&target)?]
    } else {
        config.all_sources()?
    };

    // Always include c_files when using find_source (main override / explicit name)
    if name.is_some() || config.sources.main.is_some() {
        let src_dir = config.source_dir()?;
        for c_file in &config.sources.c_files {
            let path = src_dir.join(c_file);
            if !path.exists() {
                bail!(
                    "C source '{}' (from [sources] c_files) not found in {}",
                    c_file,
                    src_dir.display()
                );
            }
            sources.push(path);
        }
    }

    if sources.is_empty() {
        bail!("No source files found.");
    }

    let mut objects = Vec::new();

    for src in &sources {
        let obj = obj_path(&build_dir, src);
        match src.extension().and_then(|e| e.to_str()) {
            Some("c") => compile_c(config, src, &obj, verbose)?,
            _ => compile_asm(config, src, &obj, verbose)?,
        }
        objects.push(obj);
    }

    let elf = config.elf_path(&target)?;
    link(config, &objects, &elf, verbose)?;

    let elapsed = start.elapsed();
    println!(
        "{:>12} in {:.2}s",
        "Finished".green().bold(),
        elapsed.as_secs_f64()
    );

    Ok(())
}

fn obj_path(build_dir: &Path, src: &Path) -> PathBuf {
    // ponytail: stem.ext.o naming prevents main.c / main.s collision
    let name = src.file_name().unwrap().to_string_lossy();
    build_dir.join(format!("{name}.o"))
}

fn compile_asm(config: &Config, src: &Path, obj: &Path, verbose: bool) -> Result<()> {
    println!(
        "{:>12} {}",
        "Compiling".green().bold(),
        src.file_name().unwrap().to_string_lossy()
    );

    let mut cmd = Command::new(&config.toolchain.cc);
    cmd.arg("-c");
    cmd.arg(format!("-march={}", config.target.arch));
    cmd.arg(format!("-mabi={}", config.target.abi));
    if config.compile.generate_debug_symbols {
        cmd.arg("-g");
    }
    for flag in &config.build.assembler_flags {
        cmd.arg(flag);
    }
    cmd.arg(src);
    cmd.arg("-o");
    cmd.arg(obj);

    run_command(
        &mut cmd,
        verbose,
        &config.toolchain.cc,
        "Assembly compilation",
    )
}

fn compile_c(config: &Config, src: &Path, obj: &Path, verbose: bool) -> Result<()> {
    println!(
        "{:>12} {}",
        "Compiling".green().bold(),
        src.file_name().unwrap().to_string_lossy()
    );

    let mut cmd = Command::new(&config.toolchain.cc);
    cmd.arg("-c");
    cmd.arg(format!("-march={}", config.target.arch));
    cmd.arg(format!("-mabi={}", config.target.abi));
    cmd.arg(format!("-O{}", config.build.optimization));
    if config.compile.generate_debug_symbols {
        cmd.arg("-g");
    }
    for flag in &config.build.compiler_flags {
        cmd.arg(flag);
    }
    cmd.arg(src);
    cmd.arg("-o");
    cmd.arg(obj);

    run_command(&mut cmd, verbose, &config.toolchain.cc, "C compilation")
}

fn link(config: &Config, objects: &[PathBuf], elf: &Path, verbose: bool) -> Result<()> {
    println!(
        "{:>12} {}",
        "Linking".green().bold(),
        elf.file_name().unwrap().to_string_lossy()
    );

    let mut cmd = Command::new(&config.toolchain.cc);

    match config.link.driver {
        LinkDriver::Ld => {
            cmd.arg("-nostdlib");
        }
        LinkDriver::Cc => {}
    }

    cmd.arg(format!("-march={}", config.target.arch));
    cmd.arg(format!("-mabi={}", config.target.abi));

    if config.build.static_link {
        cmd.arg("-static");
    }

    if let Some(script) = &config.link.script {
        cmd.arg(format!("-T{}", script));
    }

    for path in &config.link.library_paths {
        cmd.arg(format!("-L{}", path));
    }

    cmd.arg("-o");
    cmd.arg(elf);
    cmd.args(objects);

    for lib in &config.link.libraries {
        cmd.arg(format!("-l{}", lib));
    }

    for flag in &config.build.linker_flags {
        cmd.arg(flag);
    }

    run_command(&mut cmd, verbose, &config.toolchain.cc, "Linking")
}

fn run_command(cmd: &mut Command, verbose: bool, tool: &str, stage: &str) -> Result<()> {
    if verbose {
        print_command(cmd);
    }

    let output = cmd
        .stderr(Stdio::piped())
        .stdout(Stdio::piped())
        .output()
        .with_context(|| {
            format!(
                "Compiler not found: {tool}\n\n\
                 Install the required toolchain or update [toolchain] in rv.toml"
            )
        })?;

    if !output.status.success() {
        eprintln!("\n{stage} failed.");
        if verbose {
            eprintln!("\nCommand:");
            eprint_command(cmd);
        }
        eprintln!("\nOutput:");
        std::io::stdout().write_all(&output.stdout)?;
        std::io::stderr().write_all(&output.stderr)?;
        bail!("{stage} failed.");
    }

    Ok(())
}

fn print_command(cmd: &Command) {
    let prog = cmd.get_program().to_string_lossy();
    let args: Vec<_> = cmd
        .get_args()
        .map(|a| a.to_string_lossy().to_string())
        .collect();
    eprintln!("\n  {prog} \\");
    for (i, arg) in args.iter().enumerate() {
        if i == args.len() - 1 {
            eprintln!("    {arg}");
        } else {
            eprintln!("    {arg} \\");
        }
    }
}

fn eprint_command(cmd: &Command) {
    let prog = cmd.get_program().to_string_lossy();
    let args: Vec<_> = cmd
        .get_args()
        .map(|a| a.to_string_lossy().to_string())
        .collect();
    eprintln!("  {prog} \\");
    for (i, arg) in args.iter().enumerate() {
        if i == args.len() - 1 {
            eprintln!("    {arg}");
        } else {
            eprintln!("    {arg} \\");
        }
    }
}
