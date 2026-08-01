use anyhow::Result;

use crate::compiler;
use crate::config::Config;

pub fn run(file: Option<&str>, verbose: bool) -> Result<()> {
    let config = Config::load_or_adhoc(file)?;
    compiler::gcc::run(&config, verbose)
}
