use anyhow::Result;

use crate::compiler;
use crate::config::Config;
use crate::gdb;

pub fn run(verbose: bool) -> Result<()> {
    let config = Config::load()?;
    compiler::gcc::run(&config, verbose)?;
    let elf = config.elf_path(&config.resolve_target(None))?;
    gdb::debug::run(&config, &elf)
}
