use anyhow::Result;

use crate::compiler;
use crate::config::Config;
use crate::qemu;

pub fn run(name: Option<&str>) -> Result<()> {
    let config = Config::load()?;
    let target = config.resolve_target(name);
    compiler::gcc::run(&config, Some(&target))?;
    let elf = config.elf_path(&target)?;
    qemu::run::run(&config, &elf)
}
