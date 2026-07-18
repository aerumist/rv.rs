use anyhow::Result;

use crate::compiler;
use crate::config::Config;
use crate::gdb;

pub fn run(name: Option<&str>, verbose: bool) -> Result<()> {
    let config = Config::load()?;
    let target = config.resolve_target(name);
    compiler::gcc::run(&config, Some(&target), verbose)?;
    let elf = config.elf_path(&target)?;
    gdb::debug::run(&config, &elf)
}
