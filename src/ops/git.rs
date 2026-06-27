use anyhow::{Context, Result};
use tokio::process::Command;

use crate::state::data::BranchEntry;

pub async fn pull_branch(entry: &BranchEntry) -> Result<()> {
    let status = Command::new("git")
        .args(["pull", "--ff-only"])
        .current_dir(&entry.dir)
        .status()
        .await
        .with_context(|| format!("git pull in {}", entry.dir))?;
    if !status.success() {
        anyhow::bail!("git pull failed in {}", entry.dir);
    }
    Ok(())
}

pub async fn get_remote_branches(repo_url: &str) -> Result<Vec<String>> {
    let output = Command::new("git")
        .args(["ls-remote", "--heads", repo_url])
        .output()
        .await
        .context("git ls-remote failed")?;
    if !output.status.success() {
        anyhow::bail!("git ls-remote failed: {}", String::from_utf8_lossy(&output.stderr));
    }
    let stdout = String::from_utf8_lossy(&output.stdout);
    let branches = stdout
        .lines()
        .filter_map(|l| l.split('\t').nth(1))
        .filter_map(|r| r.strip_prefix("refs/heads/"))
        .map(|s| s.to_string())
        .collect();
    Ok(branches)
}

pub async fn branch_diff(entry: &BranchEntry, ref_: &str) -> Result<String> {
    let output = Command::new("git")
        .args(["diff", ref_])
        .current_dir(&entry.dir)
        .output()
        .await
        .with_context(|| format!("git diff in {}", entry.dir))?;
    Ok(String::from_utf8_lossy(&output.stdout).to_string())
}

pub async fn get_branch_status(entry: &BranchEntry) -> Result<(Option<u32>, Option<u32>, bool)> {
    let output = Command::new("git")
        .args(["status", "--porcelain=v1", "-b"])
        .current_dir(&entry.dir)
        .output()
        .await
        .with_context(|| format!("git status in {}", entry.dir))?;
    let stdout = String::from_utf8_lossy(&output.stdout);
    let mut ahead = None;
    let mut behind = None;
    let mut dirty = false;
    for line in stdout.lines() {
        if line.starts_with("##") {
            if let Some(cap) = line.find("ahead ") {
                let rest = &line[cap + 6..];
                ahead = rest.split(|c: char| !c.is_ascii_digit()).next()
                    .and_then(|s| s.parse().ok());
            }
            if let Some(cap) = line.find("behind ") {
                let rest = &line[cap + 7..];
                behind = rest.split(|c: char| !c.is_ascii_digit()).next()
                    .and_then(|s| s.parse().ok());
            }
        } else if !line.is_empty() {
            dirty = true;
        }
    }
    Ok((ahead, behind, dirty))
}
