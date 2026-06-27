use anyhow::{Context, Result};
use std::path::PathBuf;

use crate::state::config::GitRc;

pub fn gitrc_path() -> PathBuf {
    dirs::home_dir()
        .expect("home dir not found")
        .join(".gitrc")
}

pub fn gitrc_exists() -> bool {
    gitrc_path().exists()
}

pub fn read_gitrc() -> Result<GitRc> {
    let path = gitrc_path();
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("reading {}", path.display()))?;
    serde_json::from_str(&raw)
        .with_context(|| format!("parsing JSON in {}", path.display()))
}

pub fn write_gitrc(config: &GitRc) -> Result<()> {
    let path = gitrc_path();
    let json = serde_json::to_string_pretty(config)?;
    std::fs::write(&path, format!("{json}\n"))
        .with_context(|| format!("writing {}", path.display()))
}
