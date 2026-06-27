use anyhow::{Context, Result};
use chrono::Local;
use std::path::Path;

use crate::state::{
    config::ProfileSettings,
    data::{BranchEntry, BranchMode, BranchSlot},
};
use crate::util::paths::resolve_path;

pub fn read_branches_from_dir(dir: &str, slot: BranchSlot) -> Result<Vec<BranchEntry>> {
    let resolved = resolve_path(dir);
    let path = Path::new(&resolved);
    if !path.exists() {
        return Ok(Vec::new());
    }
    let mut entries = Vec::new();
    for entry in std::fs::read_dir(path)
        .with_context(|| format!("reading branch dir {}", path.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let meta = entry.metadata()?;
        let modified = meta.modified()?;
        let last_modified = chrono::DateTime::<Local>::from(modified);
        entries.push(BranchEntry {
            name: entry.file_name().to_string_lossy().to_string(),
            slot,
            dir: entry.path().to_string_lossy().to_string(),
            last_modified,
        });
    }
    entries.sort_by(|a, b| b.last_modified.cmp(&a.last_modified));
    Ok(entries)
}

pub fn list_branches(settings: &ProfileSettings, mode: BranchMode) -> Result<Vec<BranchEntry>> {
    let mut results = Vec::new();
    if mode == BranchMode::Dev || mode == BranchMode::Both {
        results.extend(read_branches_from_dir(&settings.paths.dev, BranchSlot::Dev)?);
    }
    if mode == BranchMode::Pr || mode == BranchMode::Both {
        results.extend(read_branches_from_dir(&settings.paths.pr, BranchSlot::Pr)?);
    }
    Ok(results)
}

pub fn delete_branch(entry: &BranchEntry) -> Result<()> {
    std::fs::remove_dir_all(&entry.dir)
        .with_context(|| format!("deleting branch dir {}", entry.dir))
}
