use anyhow::{Context, Result};
use std::io::Write;
use std::path::PathBuf;

use crate::state::data::AuditEntry;

pub fn audit_log_path() -> PathBuf {
    dirs::home_dir()
        .expect("home dir not found")
        .join(".gitrc-log.jsonl")
}

pub fn append_audit_log(entry: &AuditEntry) -> Result<()> {
    let path = audit_log_path();
    let mut file = std::fs::OpenOptions::new()
        .create(true)
        .append(true)
        .open(&path)
        .with_context(|| format!("opening {}", path.display()))?;
    let line = serde_json::to_string(entry)?;
    writeln!(file, "{line}")?;
    Ok(())
}

pub fn read_audit_log() -> Result<Vec<AuditEntry>> {
    let path = audit_log_path();
    if !path.exists() {
        return Ok(Vec::new());
    }
    let raw = std::fs::read_to_string(&path)
        .with_context(|| format!("reading {}", path.display()))?;
    let entries = raw
        .lines()
        .filter(|l| !l.is_empty())
        .filter_map(|l| serde_json::from_str(l).ok())
        .collect();
    Ok(entries)
}
