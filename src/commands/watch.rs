use std::sync::mpsc::channel;
use std::time::Duration;

use anyhow::Result;
use colored::Colorize;
use notify_debouncer_mini::new_debouncer;

use crate::compiler;
use crate::config::Config;

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

    if let Err(e) = compiler::gcc::run(&config, None, false) {
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
                    println!("\n{:>12} change detected", "Rebuild".yellow().bold());
                    if let Err(e) = compiler::gcc::run(&config, None, false) {
                        eprintln!("{:>12} {e}", "Error".red().bold());
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
