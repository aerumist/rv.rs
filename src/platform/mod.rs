use anyhow::{Result, bail};
use colored::Colorize;

use crate::config::Config;

struct AllowedConfig {
    arch: &'static str,
    abi: &'static str,
}

struct PlatformRules {
    name: &'static str,
    toolchain_pattern: &'static str,
    allowed: &'static [AllowedConfig],
    suggestion: &'static str,
}

// ponytail: static table, add new platforms here
static PLATFORMS: &[PlatformRules] = &[PlatformRules {
    name: "linux",
    toolchain_pattern: "linux-gnu-gcc",
    allowed: &[AllowedConfig {
        arch: "rv64gc",
        abi: "lp64d",
    }],
    suggestion: "Either:\n  - switch to rv64gc/lp64d\n  - or use a bare-metal toolchain instead.",
}];

pub fn validate(config: &Config) -> Result<()> {
    let cc = &config.toolchain.cc;

    let Some(platform) = PLATFORMS.iter().find(|p| cc.contains(p.toolchain_pattern)) else {
        return Ok(()); // bare-metal or unknown toolchain — no restriction
    };

    let arch = &config.target.arch;
    let abi = &config.target.abi;

    if platform
        .allowed
        .iter()
        .any(|a| a.arch == arch && a.abi == abi)
    {
        return Ok(());
    }

    let allowed_list = platform
        .allowed
        .iter()
        .map(|a| format!("    ISA : {}\n    ABI : {}", a.arch, a.abi))
        .collect::<Vec<_>>()
        .join("\n\n");

    bail!(
        "{}\n\n\
         Toolchain:\n    {}\n\n\
         Requested:\n    ISA : {}\n    ABI : {}\n\n\
         This {} toolchain only supports:\n\n{}\n\n\
         {}",
        "Target configuration is incompatible with the selected toolchain."
            .red()
            .bold(),
        cc,
        arch,
        abi,
        platform.name,
        allowed_list,
        platform.suggestion,
    );
}
