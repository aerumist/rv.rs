use std::io::Write;
use std::process::{Command, Stdio};

use anyhow::{Context, Result};
use colored::Colorize;

use crate::compiler;
use crate::config::Config;

pub fn run(verbose: bool) -> Result<()> {
    let config = Config::load()?;
    let target = config.resolve_target(None);
    compiler::gcc::run(&config, verbose)?;
    let elf = config.elf_path(&target)?;

    if verbose {
        println!(
            "{:>12} {} -SW {}",
            "Running".dimmed(),
            config.toolchain.readelf,
            elf.display()
        );
    }

    println!("{:>12} {}", "Memmap".cyan().bold(), elf.display());

    let output = Command::new(&config.toolchain.readelf)
        .args(["-S", "-W"])
        .arg(&elf)
        .stderr(Stdio::piped())
        .output()
        .with_context(|| format!("Failed to run readelf '{}'.", config.toolchain.readelf))?;

    if !output.status.success() {
        std::io::stderr().write_all(&output.stderr)?;
        anyhow::bail!("readelf failed.");
    }

    let stdout = String::from_utf8_lossy(&output.stdout);
    let sections = parse_sections(&stdout);

    if sections.is_empty() {
        println!("  No loadable sections found.");
        return Ok(());
    }

    // Table output
    println!();
    println!(
        "  {:<20} {:<18} {:<18} {:>10}  {}",
        "Section", "Start", "End", "Size", "Flags"
    );
    println!("  {:-<20} {:-<18} {:-<18} {:-<10}  {:-<6}", "", "", "", "", "");

    for s in &sections {
        println!(
            "  {:<20} 0x{:016X} 0x{:016X} {:>8}B  {}",
            s.name, s.addr, s.end(), s.size, s.flags
        );
    }

    // ASCII memory map
    let min_addr = sections.iter().map(|s| s.addr).min().unwrap();
    let max_addr = sections.iter().map(|s| s.end()).max().unwrap();
    let span = max_addr - min_addr;
    if span == 0 {
        return Ok(());
    }

    let bar_width = 50u64;
    println!();
    println!("  Memory Map:");
    println!("  0x{:08X} ┌{:─>width$}┐", min_addr, "", width = bar_width as usize);

    for s in &sections {
        let start_off = ((s.addr - min_addr) * bar_width / span).max(0);
        let end_off = ((s.end() - min_addr) * bar_width / span).max(start_off + 1);
        let bar_len = (end_off - start_off) as usize;
        let padding = start_off as usize;
        println!(
            "  0x{:08X} │{:pad$}{:█<len$}{:pad2$}│ {}",
            s.addr,
            "",
            "",
            "",
            format!("{} ({})", s.name, human_size(s.size)),
            pad = padding,
            len = bar_len,
            pad2 = (bar_width as usize) - padding - bar_len,
        );
    }

    println!("  0x{:08X} └{:─>width$}┘", max_addr, "", width = bar_width as usize);

    Ok(())
}

struct Section {
    name: String,
    addr: u64,
    size: u64,
    flags: String,
}

impl Section {
    fn end(&self) -> u64 {
        self.addr + self.size
    }
}

fn human_size(bytes: u64) -> String {
    if bytes < 1024 {
        format!("{bytes}B")
    } else if bytes < 1024 * 1024 {
        format!("{:.1}K", bytes as f64 / 1024.0)
    } else {
        format!("{:.1}M", bytes as f64 / (1024.0 * 1024.0))
    }
}

/// Parse `readelf -SW` output into structured sections.
/// Only keeps sections with the ALLOC flag (A) — those that occupy memory.
fn parse_sections(raw: &str) -> Vec<Section> {
    let mut sections = Vec::new();

    for line in raw.lines() {
        let line = line.trim();
        // readelf section lines look like:
        //   [Nr] Name          Type     Address          Off      Size   ES Flg Lk Inf Al
        //   [ 0]               NULL     0000000000000000 000000 000000 00      0   0  0
        // We need the Name, Address, Size, and Flags columns.
        if !line.starts_with('[') {
            continue;
        }

        // Split on whitespace, skip the [Nr] token
        let tokens: Vec<&str> = line.split_whitespace().collect();
        if tokens.len() < 8 {
            continue;
        }

        let name = tokens[1].to_string();
        let addr = u64::from_str_radix(tokens[3].trim_start_matches("0x"), 16).unwrap_or(0);
        // Size is at index 5 for 64-bit, but readelf -W uses a different layout.
        // With -W (wide), the columns are: [Nr] Name Type Address Off Size ES Flg Lk Inf Al
        let size = u64::from_str_radix(tokens[5].trim_start_matches("0x"), 16).unwrap_or(0);
        let flags = tokens[6].to_string();

        // Only include sections that are allocated in memory
        if flags.contains('A') {
            sections.push(Section {
                name,
                addr,
                size,
                flags,
            });
        }
    }

    sections.sort_by_key(|s| s.addr);
    sections
}
