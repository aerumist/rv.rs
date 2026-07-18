use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub project: Project,
    #[serde(default)]
    pub target: Target,
    #[serde(default)]
    pub sources: Sources,
    #[serde(default)]
    pub toolchain: Toolchain,
    #[serde(default)]
    pub build: Build,
    #[serde(default)]
    pub link: Link,
    #[serde(default)]
    pub compile: Compile,
    #[serde(default)]
    pub output: Output,
    #[serde(default)]
    pub qemu: Qemu,
}

#[derive(Debug, Deserialize)]
pub struct Project {
    pub name: String,
}

#[derive(Debug, Deserialize)]
pub struct Target {
    #[serde(default = "default_arch")]
    pub arch: String,
    #[serde(default = "default_abi")]
    pub abi: String,
}

impl Default for Target {
    fn default() -> Self {
        Self { arch: default_arch(), abi: default_abi() }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct Sources {
    pub main: Option<String>,
    #[serde(default)]
    pub c_files: Vec<String>,
}

#[derive(Debug, Deserialize)]
pub struct Toolchain {
    #[serde(default = "default_cc")]
    pub cc: String,
    #[serde(default = "default_objdump")]
    pub objdump: String,
    #[serde(default = "default_nm")]
    pub nm: String,
    #[serde(default = "default_readelf")]
    pub readelf: String,
    #[serde(default = "default_gdb")]
    pub gdb: String,
}

impl Default for Toolchain {
    fn default() -> Self {
        Self {
            cc: default_cc(),
            objdump: default_objdump(),
            nm: default_nm(),
            readelf: default_readelf(),
            gdb: default_gdb(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Build {
    #[serde(default = "default_opt_level")]
    pub optimization: String,
    #[serde(default, rename = "static")]
    pub static_link: bool,
    #[serde(default)]
    pub compiler_flags: Vec<String>,
    #[serde(default)]
    pub assembler_flags: Vec<String>,
    #[serde(default)]
    pub linker_flags: Vec<String>,
}

impl Default for Build {
    fn default() -> Self {
        Self {
            optimization: default_opt_level(),
            static_link: false,
            compiler_flags: Vec::new(),
            assembler_flags: Vec::new(),
            linker_flags: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Default)]
pub struct Link {
    #[serde(default = "default_link_driver")]
    pub driver: LinkDriver,
    #[serde(default)]
    pub libraries: Vec<String>,
    #[serde(default)]
    pub library_paths: Vec<String>,
    pub script: Option<String>,
}

#[derive(Debug, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum LinkDriver {
    #[default]
    Ld,
    Cc,
}

fn default_link_driver() -> LinkDriver { LinkDriver::Ld }

#[derive(Debug, Deserialize)]
pub struct Compile {
    #[serde(default)]
    pub generate_debug_symbols: bool,
}

impl Default for Compile {
    fn default() -> Self {
        Self { generate_debug_symbols: false }
    }
}

#[derive(Debug, Deserialize)]
pub struct Output {
    #[serde(default = "default_build_dir")]
    pub directory: String,
    pub binary: Option<String>,
}

impl Default for Output {
    fn default() -> Self {
        Self { directory: default_build_dir(), binary: None }
    }
}

#[derive(Debug, Deserialize)]
pub struct Qemu {
    #[serde(default = "default_qemu_mode")]
    pub mode: QemuMode,
    #[serde(default = "default_qemu_binary")]
    pub binary: String,
    #[serde(default)]
    pub args: Vec<String>,
}

impl Default for Qemu {
    fn default() -> Self {
        Self {
            mode: default_qemu_mode(),
            binary: default_qemu_binary(),
            args: Vec::new(),
        }
    }
}

#[derive(Debug, Deserialize, Default, Clone, Copy, PartialEq, Eq)]
#[serde(rename_all = "lowercase")]
pub enum QemuMode {
    #[default]
    User,
    System,
}

fn default_arch() -> String { "rv64imac".into() }
fn default_abi() -> String { "lp64".into() }
fn default_cc() -> String { "riscv64-elf-gcc".into() }
fn default_objdump() -> String { "riscv64-elf-objdump".into() }
fn default_nm() -> String { "riscv64-elf-nm".into() }
fn default_readelf() -> String { "riscv64-elf-readelf".into() }
fn default_gdb() -> String { "riscv64-elf-gdb".into() }
fn default_qemu_binary() -> String { "qemu-riscv64".into() }
fn default_qemu_mode() -> QemuMode { QemuMode::User }
fn default_build_dir() -> String { "build".into() }
fn default_opt_level() -> String { "0".into() }

impl Config {
    pub fn load() -> Result<Self> {
        let path = Self::find_config()?;
        let content = std::fs::read_to_string(&path)
            .with_context(|| format!("Failed to read {}", path.display()))?;
        let config: Config = toml::from_str(&content)
            .with_context(|| format!("Failed to parse {}", path.display()))?;
        Ok(config)
    }

    fn find_config() -> Result<PathBuf> {
        let cwd = std::env::current_dir()?;
        let mut dir = cwd.as_path();
        loop {
            let candidate = dir.join("rv.toml");
            if candidate.exists() {
                return Ok(candidate);
            }
            match dir.parent() {
                Some(parent) => dir = parent,
                None => bail!(
                    "No rv.toml found.\n\
                     Run `rv new <name>` to create a project, or add rv.toml to the current directory."
                ),
            }
        }
    }

    pub fn project_root(&self) -> Result<PathBuf> {
        let path = Self::find_config()?;
        Ok(path.parent().unwrap().to_path_buf())
    }

    pub fn source_dir(&self) -> Result<PathBuf> {
        Ok(self.project_root()?.join("src"))
    }

    pub fn build_dir(&self) -> Result<PathBuf> {
        Ok(self.project_root()?.join(&self.output.directory))
    }

    pub fn elf_path(&self, name: &str) -> Result<PathBuf> {
        let dir = self.build_dir()?;
        match &self.output.binary {
            Some(bin) => Ok(dir.join(bin)),
            None => Ok(dir.join(format!("{name}.elf"))),
        }
    }

    pub fn resolve_target(&self, name: Option<&str>) -> String {
        name.unwrap_or(&self.project.name).to_string()
    }

    pub fn find_source(&self, name: &str) -> Result<PathBuf> {
        let src = self.source_dir()?;
        if let Some(main) = &self.sources.main {
            let path = src.join(main);
            if path.exists() {
                return Ok(path);
            }
            bail!("Source file '{}' (from [sources] main) not found in {}", main, src.display());
        }
        for ext in &["S", "s", "asm"] {
            let path = src.join(format!("{name}.{ext}"));
            if path.exists() {
                return Ok(path);
            }
        }
        bail!(
            "No source file found for '{name}'.\n\
             Looked for: {name}.S, {name}.s, {name}.asm in {}",
            src.display()
        )
    }

    pub fn all_sources(&self) -> Result<Vec<PathBuf>> {
        let src = self.source_dir()?;
        if !src.exists() {
            bail!("Source directory '{}' does not exist.", src.display());
        }
        let mut files = Vec::new();
        for entry in walkdir::WalkDir::new(&src)
            .into_iter()
            .filter_map(|e| e.ok())
        {
            let path = entry.path();
            if let Some(ext) = path.extension() {
                if matches!(ext.to_str(), Some("S" | "s" | "asm")) {
                    files.push(path.to_path_buf());
                }
            }
        }
        for c_file in &self.sources.c_files {
            let path = src.join(c_file);
            if !path.exists() {
                bail!("C source '{}' (from [sources] c_files) not found in {}", c_file, src.display());
            }
            files.push(path);
        }
        if files.is_empty() {
            bail!("No source files found in {}.", src.display());
        }
        files.sort();
        Ok(files)
    }
}
