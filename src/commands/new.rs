use std::fs;
use std::path::Path;
use std::process::Command;

use anyhow::{Result, bail};
use colored::Colorize;
use dialoguer::{Input, Select, theme::ColorfulTheme};

use crate::templates;

const KNOWN_COMPILERS: &[&str] = &[
    "riscv64-elf-gcc",
    "riscv64-unknown-elf-gcc",
    "riscv64-linux-gnu-gcc",
    "riscv32-elf-gcc",
    "riscv32-unknown-elf-gcc",
];

fn detect_compilers() -> Vec<String> {
    KNOWN_COMPILERS
        .iter()
        .filter(|name| {
            Command::new("which")
                .arg(name)
                .output()
                .map(|o| o.status.success())
                .unwrap_or(false)
        })
        .map(|s| s.to_string())
        .collect()
}

fn toolchain_prefix(compiler: &str) -> &str {
    compiler.strip_suffix("gcc").unwrap_or("riscv64-elf-")
}

pub fn run(name: &str, template: &str) -> Result<()> {
    let path = Path::new(name);

    if path.exists() {
        bail!("Directory '{name}' already exists.");
    }

    let theme = ColorfulTheme::default();

    // --- Header ---
    println!();
    println!(
        "  {}  {}",
        "▸".cyan().bold(),
        format!("Creating project: {name}").bold()
    );
    println!();

    // --- Template selection ---
    let templates_list = &["default — assembly only", "qemu — mixed ASM + C (QEMU user mode)"];
    let template_idx = if template != "default" {
        match template {
            "qemu" => 1,
            _ => bail!("Unknown template '{template}'. Available: default, qemu"),
        }
    } else {
        Select::with_theme(&theme)
            .with_prompt("Template")
            .items(templates_list)
            .default(0)
            .interact()?
    };

    // --- Compiler detection ---
    let installed = detect_compilers();

    println!();
    if installed.is_empty() {
        println!(
            "  {}  No RISC-V compilers found on PATH",
            "⚠".yellow().bold()
        );
    } else {
        println!(
            "  {}  Found {} compiler{}:",
            "✓".green().bold(),
            installed.len(),
            if installed.len() == 1 { "" } else { "s" }
        );
        for cc in &installed {
            println!("      {}", cc.dimmed());
        }
    }
    println!();

    let compiler = if installed.len() > 1 {
        let idx = Select::with_theme(&theme)
            .with_prompt("Compiler")
            .items(&installed)
            .default(0)
            .interact()?;
        installed[idx].clone()
    } else if installed.len() == 1 {
        let cc = &installed[0];
        println!("  {}  Using {}", "→".cyan(), cc.bold());
        cc.clone()
    } else {
        Input::with_theme(&theme)
            .with_prompt("Compiler (e.g. riscv64-elf-gcc)")
            .default("riscv64-elf-gcc".to_string())
            .interact_text()?
    };

    // --- Architecture ---
    let archs = &["rv64imac", "rv64gc", "rv32imac", "rv32i"];
    let arch_idx = Select::with_theme(&theme)
        .with_prompt("Architecture")
        .items(archs)
        .default(0)
        .interact()?;
    let arch = archs[arch_idx];

    // --- ABI ---
    let abi = match arch {
        a if a.starts_with("rv64") => "lp64",
        _ => "ilp32",
    };

    // --- QEMU mode ---
    let qemu_modes = &["user", "system"];
    let qemu_idx = Select::with_theme(&theme)
        .with_prompt("QEMU mode")
        .items(qemu_modes)
        .default(0)
        .interact()?;
    let qemu_mode = qemu_modes[qemu_idx];

    let qemu_binary = match (qemu_mode, arch) {
        ("user", a) if a.starts_with("rv64") => "qemu-riscv64",
        ("user", _) => "qemu-riscv32",
        ("system", a) if a.starts_with("rv64") => "qemu-system-riscv64",
        ("system", _) => "qemu-system-riscv32",
        _ => "qemu-riscv64",
    };

    // --- Build project ---
    println!();
    println!("  {}  Scaffolding...", "⏳".dimmed());

    fs::create_dir_all(path.join("src"))?;
    fs::create_dir_all(path.join("build"))?;

    let prefix = toolchain_prefix(&compiler);
    let config = ProjectConfig {
        name: name.to_string(),
        compiler: compiler.clone(),
        prefix: prefix.to_string(),
        arch: arch.to_string(),
        abi: abi.to_string(),
        qemu_mode: qemu_mode.to_string(),
        qemu_binary: qemu_binary.to_string(),
    };

    match template_idx {
        0 => {
            fs::write(path.join("rv.toml"), render_toml(&config, false))?;
            fs::write(
                path.join("src").join(format!("{name}.S")),
                templates::starter_asm(name),
            )?;
        }
        1 => {
            fs::write(path.join("rv.toml"), render_toml(&config, true))?;
            fs::write(
                path.join("src").join("main.S"),
                templates::starter_asm_qemu(name),
            )?;
            fs::write(
                path.join("src").join("helper.c"),
                templates::starter_c_qemu(),
            )?;
        }
        _ => unreachable!(),
    }

    fs::write(path.join(".gitignore"), templates::gitignore())?;

    // --- Summary ---
    println!("\r  {}  {}", "✓".green().bold(), "Done!".green().bold());
    println!();
    println!("  ┌─────────────────────────────────────────┐");
    println!(
        "  │  {}  {:<35}│",
        "📁".dimmed(),
        name.bold()
    );
    println!("  ├─────────────────────────────────────────┤");
    println!("  │  Compiler    {}│", format!("{:<27}", compiler));
    println!("  │  Arch        {}│", format!("{:<27}", arch));
    println!("  │  ABI         {}│", format!("{:<27}", abi));
    println!("  │  QEMU        {}│", format!("{:<27}", format!("{qemu_mode} ({qemu_binary})")));
    println!("  └─────────────────────────────────────────┘");
    println!();
    println!(
        "  {}",
        format!("cd {name} && rv run").dimmed()
    );
    println!();

    Ok(())
}

struct ProjectConfig {
    name: String,
    compiler: String,
    prefix: String,
    arch: String,
    abi: String,
    qemu_mode: String,
    qemu_binary: String,
}

fn render_toml(cfg: &ProjectConfig, mixed: bool) -> String {
    let sources = if mixed {
        format!(
            "\n[sources]\nmain = \"main.S\"\nc_files = [\"helper.c\"]\n"
        )
    } else {
        String::new()
    };

    let link = if mixed {
        "\n[link]\ndriver = \"cc\"\n".to_string()
    } else {
        String::new()
    };

    format!(
        r#"[project]
name = "{name}"

[target]
arch = "{arch}"
abi = "{abi}"
{sources}
[toolchain]
cc = "{cc}"
objdump = "{prefix}objdump"
nm = "{prefix}nm"
readelf = "{prefix}readelf"
gdb = "{prefix}gdb"

[build]
optimization = "0"
{link}
[output]
directory = "build"

[qemu]
mode = "{qemu_mode}"
binary = "{qemu_binary}"
"#,
        name = cfg.name,
        arch = cfg.arch,
        abi = cfg.abi,
        sources = sources,
        cc = cfg.compiler,
        prefix = cfg.prefix,
        link = link,
        qemu_mode = cfg.qemu_mode,
        qemu_binary = cfg.qemu_binary,
    )
}
