use std::path::PathBuf;

use anyhow::{Context, Result, bail};
use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct Config {
    pub project: Project,
    #[serde(default)]
    pub target: Target,
    #[serde(default)]
    pub compiler: Compiler,
    #[serde(default)]
    pub qemu: Qemu,
    #[serde(default)]
    pub paths: Paths,
    #[serde(default)]
    pub build: Build,
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
        Self {
            arch: default_arch(),
            abi: default_abi(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Compiler {
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

impl Default for Compiler {
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
pub struct Qemu {
    #[serde(default = "default_qemu_user")]
    pub user: String,
    #[serde(default = "default_qemu_system")]
    pub system: String,
    #[serde(default = "default_qemu_mode")]
    pub mode: QemuMode,
}

impl Default for Qemu {
    fn default() -> Self {
        Self {
            user: default_qemu_user(),
            system: default_qemu_system(),
            mode: default_qemu_mode(),
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

#[derive(Debug, Deserialize)]
pub struct Paths {
    #[serde(default = "default_source")]
    pub source: String,
    #[serde(default = "default_build")]
    pub build: String,
}

impl Default for Paths {
    fn default() -> Self {
        Self {
            source: default_source(),
            build: default_build(),
        }
    }
}

#[derive(Debug, Deserialize)]
pub struct Build {
    #[serde(default = "default_opt_level")]
    pub opt_level: String,
}

impl Default for Build {
    fn default() -> Self {
        Self {
            opt_level: default_opt_level(),
        }
    }
}

fn default_arch() -> String { "rv64imac".into() }
fn default_abi() -> String { "lp64".into() }
fn default_cc() -> String { "riscv64-elf-gcc".into() }
fn default_objdump() -> String { "riscv64-elf-objdump".into() }
fn default_nm() -> String { "riscv64-elf-nm".into() }
fn default_readelf() -> String { "riscv64-elf-readelf".into() }
fn default_gdb() -> String { "riscv64-elf-gdb".into() }
fn default_qemu_user() -> String { "qemu-riscv64".into() }
fn default_qemu_system() -> String { "qemu-system-riscv64".into() }
fn default_qemu_mode() -> QemuMode { QemuMode::User }
fn default_source() -> String { "src".into() }
fn default_build() -> String { "build".into() }
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
        Ok(self.project_root()?.join(&self.paths.source))
    }

    pub fn build_dir(&self) -> Result<PathBuf> {
        Ok(self.project_root()?.join(&self.paths.build))
    }

    pub fn elf_path(&self, name: &str) -> Result<PathBuf> {
        Ok(self.build_dir()?.join(format!("{name}.elf")))
    }

    /// Resolve target name: use explicit name, or fall back to project name
    pub fn resolve_target(&self, name: Option<&str>) -> String {
        name.unwrap_or(&self.project.name).to_string()
    }

    /// Find the source file for a given target name
    pub fn find_source(&self, name: &str) -> Result<PathBuf> {
        let src = self.source_dir()?;
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

    /// Collect all assembly source files
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
        if files.is_empty() {
            bail!("No assembly files found in {}.", src.display());
        }
        files.sort();
        Ok(files)
    }

}
