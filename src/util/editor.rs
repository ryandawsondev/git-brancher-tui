use anyhow::{Context, Result};
use std::path::Path;
use std::process::Command;

pub fn open_in_editor(path: &Path) -> Result<()> {
    let editor = std::env::var("EDITOR")
        .or_else(|_| std::env::var("VISUAL"))
        .unwrap_or_else(|_| {
            if cfg!(windows) { "notepad".to_string() } else { "vi".to_string() }
        });
    Command::new(&editor)
        .arg(path)
        .status()
        .with_context(|| format!("opening editor '{editor}'"))?;
    Ok(())
}

pub fn open_in_vscode(path: &Path) -> Result<()> {
    Command::new("code")
        .arg(path)
        .status()
        .context("opening VS Code (is 'code' in PATH?)")?;
    Ok(())
}

pub fn reveal_in_file_manager(path: &Path) -> Result<()> {
    if cfg!(windows) {
        Command::new("explorer").arg(path).status()?;
    } else if cfg!(target_os = "macos") {
        Command::new("open").arg(path).status()?;
    } else {
        Command::new("xdg-open").arg(path).status()?;
    }
    Ok(())
}
