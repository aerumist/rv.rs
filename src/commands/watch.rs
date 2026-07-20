use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::sync::mpsc::channel;
use std::time::Duration;

use anyhow::Result;
use colored::Colorize;
use notify_debouncer_mini::new_debouncer;

use crate::compiler;
use crate::config::Config;

fn file_hash(path: &std::path::Path) -> Option<u64> {
    let content = std::fs::read(path).ok()?;
    let mut h = DefaultHasher::new();
    content.hash(&mut h);
    Some(h.finish())
}

pub fn run() -> Result<()> {
    let config = Config::load()?;
    let source_dir = config.source_dir()?;

    println!(
        "{:>12} {} for changes...",
        "Watching".cyan().bold(),
        source_dir.display()
    );

    let (tx, rx) = channel();
    let mut debouncer = new_debouncer(Duration::from_millis(300), tx)?;

    debouncer
        .watcher()
        .watch(&source_dir, notify::RecursiveMode::Recursive)?;

    // ponytail: snapshot source hashes so we only rebuild on actual content change,
    // not on atime/metadata events that notify-debouncer-mini can't distinguish
    let mut hashes: std::collections::HashMap<std::path::PathBuf, u64> = std::collections::HashMap::new();
    for src in config.all_sources()? {
        if let Some(h) = file_hash(&src) {
            hashes.insert(src, h);
        }
    }

    if let Err(e) = compiler::gcc::run(&config, false) {
        eprintln!("{:>12} {e}", "Error".red().bold());
    }

    loop {
        match rx.recv() {
            Ok(Ok(events)) => {
                let has_source = events.iter().any(|e| {
                    e.path
                        .extension()
                        .is_some_and(|ext| matches!(ext.to_str(), Some("S" | "s" | "asm" | "c")))
                });
                if has_source {
                    let changed = config.all_sources()?.iter().any(|src| {
                        let cur = file_hash(src);
                        let prev = hashes.get(src).copied();
                        cur != prev
                    });
                    if changed {
                        // refresh snapshot
                        hashes.clear();
                        for src in config.all_sources()? {
                            if let Some(h) = file_hash(&src) {
                                hashes.insert(src, h);
                            }
                        }
                        println!("\n{:>12} change detected", "Rebuild".yellow().bold());
                        if let Err(e) = compiler::gcc::run(&config, false) {
                            eprintln!("{:>12} {e}", "Error".red().bold());
                        }
                    }
                }
            }
            Ok(Err(errs)) => {
                eprintln!("{:>12} watch error: {errs}", "Error".red().bold());
            }
            Err(_) => break,
        }
    }

    Ok(())
}
