use std::fs;
use std::path::Path;

use anyhow::{Result, bail};
use colored::Colorize;

use crate::templates;

pub fn run(name: &str) -> Result<()> {
    let path = Path::new(name);

    if path.exists() {
        bail!("Directory '{name}' already exists.");
    }

    fs::create_dir_all(path.join("src"))?;
    fs::create_dir_all(path.join("build"))?;

    fs::write(path.join("rv.toml"), templates::rv_toml(name))?;
    fs::write(path.join("src").join(format!("{name}.S")), templates::starter_asm(name))?;
    fs::write(path.join(".gitignore"), templates::gitignore())?;

    println!("{:>12} project '{name}'", "Created".green().bold());
    println!("     {}", path.join("rv.toml").display());
    println!("     {}", path.join("src").join(format!("{name}.S")).display());
    println!("\n  cd {name} && rv build");

    Ok(())
}
