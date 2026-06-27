use anyhow::{Context, Result};
use std::path::Path;
use tokio::process::Command;

pub struct CloneOptions {
    pub repo_url: String,
    pub branch: String,
    pub target_dir: String,
    pub files: Vec<(String, String)>,
    pub pre_clone: Vec<String>,
    pub post_clone: Vec<String>,
    pub profile_dir: String,
}

pub async fn clone_branch(opts: CloneOptions) -> Result<()> {
    run_hooks(&opts.pre_clone, "pre-clone", Path::new(".")).await?;

    let status = Command::new("git")
        .args(["clone", "--branch", &opts.branch, "--single-branch", &opts.repo_url, &opts.target_dir])
        .status()
        .await
        .context("git clone failed")?;
    if !status.success() {
        anyhow::bail!("git clone failed");
    }

    inject_files(&opts.files, Path::new(&opts.profile_dir), Path::new(&opts.target_dir))?;

    run_hooks(&opts.post_clone, "post-clone", Path::new(&opts.target_dir)).await?;

    Ok(())
}

pub fn inject_files(files: &[(String, String)], profile_dir: &Path, target: &Path) -> Result<()> {
    for (source, dest) in files {
        let src_path = profile_dir.join(source);
        let dst_path = target.join(dest);
        if src_path.exists() {
            if let Some(parent) = dst_path.parent() {
                std::fs::create_dir_all(parent)?;
            }
            std::fs::copy(&src_path, &dst_path)
                .with_context(|| format!("copying {} → {}", src_path.display(), dst_path.display()))?;
        }
    }
    Ok(())
}

pub async fn run_hooks(cmds: &[String], label: &str, cwd: &Path) -> Result<()> {
    for cmd in cmds {
        let status = if cfg!(windows) {
            Command::new("cmd")
                .args(["/C", cmd])
                .current_dir(cwd)
                .status()
                .await
        } else {
            Command::new("sh")
                .args(["-c", cmd])
                .current_dir(cwd)
                .status()
                .await
        };
        let status = status.with_context(|| format!("{label} hook failed: {cmd}"))?;
        if !status.success() {
            anyhow::bail!("{label} hook failed: {cmd}");
        }
    }
    Ok(())
}
