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

#[derive(Debug, PartialEq, Eq)]
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
        let open_idx = match line.find('[') {
            Some(i) => i,
            None => continue,
        };
        let close_idx = match line[open_idx..].find(']') {
            Some(i) => open_idx + i,
            None => continue,
        };

        // Skip header lines like [Nr] or non-section brackets
        let nr_str = line[open_idx + 1..close_idx].trim();
        if nr_str.eq_ignore_ascii_case("Nr") || nr_str.parse::<u32>().is_err() {
            continue;
        }

        let rest = line[close_idx + 1..].trim();
        let tokens: Vec<&str> = rest.split_whitespace().collect();

        // Standard layout of tokens after ]:
        // With name: [Name, Type, Address, Off, Size, ES, (Flg), Lk, Inf, Al]
        // Without name: [Type, Address, Off, Size, ES, (Flg), Lk, Inf, Al]
        if tokens.len() < 5 {
            continue;
        }

        let (name, addr_idx) = if u64::from_str_radix(tokens[1].trim_start_matches("0x"), 16).is_ok()
            && !tokens[1].chars().all(|c| c.is_ascii_digit())
        {
            // tokens[1] is Type (not pure decimal), tokens[2] is Address
            (tokens[0].to_string(), 2)
        } else if u64::from_str_radix(tokens[1].trim_start_matches("0x"), 16).is_ok() {
            // tokens[1] is Address (Name is empty)
            (String::new(), 1)
        } else {
            // Fallback: tokens[0] is Name, tokens[2] is Address
            (tokens[0].to_string(), 2)
        };

        if tokens.len() < addr_idx + 3 {
            continue;
        }

        let addr = match u64::from_str_radix(tokens[addr_idx].trim_start_matches("0x"), 16) {
            Ok(val) => val,
            Err(_) => continue,
        };

        let _off = match u64::from_str_radix(tokens[addr_idx + 1].trim_start_matches("0x"), 16) {
            Ok(val) => val,
            Err(_) => continue,
        };

        let size = match u64::from_str_radix(tokens[addr_idx + 2].trim_start_matches("0x"), 16) {
            Ok(val) => val,
            Err(_) => continue,
        };

        // Flg (if present) is at addr_idx + 4
        let flags = if tokens.len() > addr_idx + 4 {
            let candidate = tokens[addr_idx + 4];
            // Flags consist of letters (e.g. AX, A, WA, MS), whereas Lk is a decimal integer
            if !candidate.chars().all(|c| c.is_ascii_digit()) {
                candidate.to_string()
            } else {
                String::new()
            }
        } else {
            String::new()
        };

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_sections_64bit() {
        let raw = r#"
There are 29 section headers, starting at offset 0x31a8:

Section Headers:
  [Nr] Name              Type            Address          Off    Size   ES Flg Lk Inf Al
  [ 0]                   NULL            0000000000000000 000000 000000 00      0   0  0
  [ 1] .text             PROGBITS        00000000000100f0 0000f0 000084 00  AX  0   0  4
  [ 2] .rodata           PROGBITS        0000000000010174 000174 000018 00   A  0   0  4
  [ 3] .data             PROGBITS        0000000000011000 001000 000010 00  WA  0   0  8
  [ 4] .bss              NOBITS          0000000000011010 001010 000020 00  WA  0   0  8
  [ 5] .comment          PROGBITS        0000000000000000 001030 00002b 01  MS  0   0  1
  [ 6] .symtab           SYMTAB          0000000000000000 001090 000630 18      8  33  8
  [10] .eh_frame         PROGBITS        0000000000010190 000190 000038 00   A  0   0  8
Key to Flags:
  W (write), A (alloc), X (execute), M (merge), S (strings), I (info),
"#;
        let sections = parse_sections(raw);
        assert_eq!(
            sections,
            vec![
                Section {
                    name: ".text".into(),
                    addr: 0x100f0,
                    size: 0x84,
                    flags: "AX".into(),
                },
                Section {
                    name: ".rodata".into(),
                    addr: 0x10174,
                    size: 0x18,
                    flags: "A".into(),
                },
                Section {
                    name: ".eh_frame".into(),
                    addr: 0x10190,
                    size: 0x38,
                    flags: "A".into(),
                },
                Section {
                    name: ".data".into(),
                    addr: 0x11000,
                    size: 0x10,
                    flags: "WA".into(),
                },
                Section {
                    name: ".bss".into(),
                    addr: 0x11010,
                    size: 0x20,
                    flags: "WA".into(),
                },
            ]
        );
    }
}
