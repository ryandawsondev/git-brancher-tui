use std::path::Path;
use tokio::process::Command;

use crate::state::{config::ProfileSettings, data::CheckResult};
use crate::util::paths::resolve_path;

pub fn check_settings_fields(s: &ProfileSettings) -> Vec<CheckResult> {
    vec![
        cr("repo.ssh is set", !s.repo.ssh.is_empty()),
        cr("repo.https is set", !s.repo.https.is_empty()),
        cr("paths.dev is set", !s.paths.dev.is_empty()),
        cr("paths.pr is set", !s.paths.pr.is_empty()),
    ]
}

pub fn check_paths(s: &ProfileSettings) -> Vec<CheckResult> {
    let dev = resolve_path(&s.paths.dev);
    let pr = resolve_path(&s.paths.pr);
    vec![
        cr(format!("paths.dev exists: '{dev}'"), Path::new(&dev).exists()),
        cr(format!("paths.pr exists: '{pr}'"), Path::new(&pr).exists()),
    ]
}

pub fn check_files(s: &ProfileSettings, profile_dir: &str) -> Vec<CheckResult> {
    if s.files.is_empty() {
        return vec![cr("No file mappings defined", true)];
    }
    s.files.iter().map(|f| {
        let src = Path::new(profile_dir).join(&f.source);
        let ok = src.exists();
        cr(
            if ok {
                format!("File exists: '{}' → '{}'", f.source, f.dest)
            } else {
                format!("File missing: '{}' (expected at '{}')", f.source, src.display())
            },
            ok,
        )
    }).collect()
}

async fn bin_on_path(cmd: &str) -> bool {
    let bin = cmd.trim().split_whitespace().next().unwrap_or(cmd);
    let checker = if cfg!(windows) { "where" } else { "which" };
    Command::new(checker)
        .arg(bin)
        .stdout(std::process::Stdio::null())
        .stderr(std::process::Stdio::null())
        .status()
        .await
        .map(|s| s.success())
        .unwrap_or(false)
}

pub async fn check_hooks(s: &ProfileSettings) -> Vec<CheckResult> {
    let mut out = Vec::new();
    for (label, hooks) in [("pre-clone", &s.pre_clone), ("post-clone", &s.post_clone)] {
        if hooks.is_empty() {
            out.push(cr(format!("No {label} hooks defined"), true));
        } else {
            out.push(cr(format!("{label} hooks ({}): {}", hooks.len(), hooks.join(", ")), true));
            for cmd in hooks {
                let found = bin_on_path(cmd).await;
                let bin = cmd.trim().split_whitespace().next().unwrap_or(cmd);
                out.push(cr(format!("  {label} '{bin}' on PATH"), found));
            }
        }
    }
    out
}

pub async fn check_reachability(s: &ProfileSettings) -> Vec<CheckResult> {
    let mut out = Vec::new();
    for (proto, url) in [("SSH", &s.repo.ssh), ("HTTPS", &s.repo.https)] {
        let ok = Command::new("git")
            .args(["ls-remote", "--exit-code", url, "HEAD"])
            .stdout(std::process::Stdio::null())
            .stderr(std::process::Stdio::null())
            .status()
            .await
            .map(|s| s.success())
            .unwrap_or(false);
        out.push(cr(
            format!("{proto} {}: {url}", if ok { "reachable" } else { "unreachable" }),
            ok,
        ));
    }
    out
}

pub async fn run_all_checks(settings: ProfileSettings, profile_dir: String) -> Vec<CheckResult> {
    let mut results = Vec::new();
    results.extend(check_settings_fields(&settings));
    results.extend(check_paths(&settings));
    results.extend(check_files(&settings, &profile_dir));
    results.extend(check_hooks(&settings).await);
    results.extend(check_reachability(&settings).await);
    results
}

fn cr(label: impl Into<String>, ok: bool) -> CheckResult {
    CheckResult { label: label.into(), ok, detail: None }
}
