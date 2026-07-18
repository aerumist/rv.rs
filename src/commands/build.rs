use anyhow::Result;

use crate::compiler;
use crate::config::Config;

pub fn run(name: Option<&str>, verbose: bool) -> Result<()> {
    let config = Config::load()?;
    compiler::gcc::run(&config, name, verbose)
}
