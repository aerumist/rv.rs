use anyhow::Result;
use colored::Colorize;

use crate::config::Config;

pub fn run() -> Result<()> {
    let config = Config::load()?;
    let build_dir = config.build_dir()?;

    if build_dir.exists() {
        std::fs::remove_dir_all(&build_dir)?;
        println!("{:>12} {}", "Removed".green().bold(), build_dir.display());
    } else {
        println!("{:>12} nothing to clean", "Clean".green().bold());
    }

    Ok(())
}
