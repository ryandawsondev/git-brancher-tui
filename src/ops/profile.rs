use anyhow::{Context, Result};
use std::path::Path;

use crate::state::{
    config::{GitRc, ProfileSettings, RepoUrls, ProfilePaths, FileMapping},
    data::{DiffEntry, ProfileSummary},
};
use crate::util::paths::resolve_path;

pub fn list_profiles(config: &GitRc) -> Result<Vec<ProfileSummary>> {
    let profiles_dir = resolve_path(&config.profiles_dir);
    let dir = Path::new(&profiles_dir);
    if !dir.exists() {
        return Ok(Vec::new());
    }
    let mut summaries = Vec::new();
    for entry in std::fs::read_dir(dir)
        .with_context(|| format!("reading profiles dir {}", dir.display()))?
    {
        let entry = entry?;
        if !entry.file_type()?.is_dir() {
            continue;
        }
        let name = entry.file_name().to_string_lossy().to_string();
        let profile_dir = entry.path().to_string_lossy().to_string();
        let settings_path = entry.path().join("settings.json");
        let settings = if settings_path.exists() {
            std::fs::read_to_string(&settings_path)
                .ok()
                .and_then(|raw| serde_json::from_str(&raw).ok())
        } else {
            None
        };
        summaries.push(ProfileSummary {
            name,
            profile_dir,
            settings,
            settings_path: settings_path.to_string_lossy().to_string(),
        });
    }
    summaries.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(summaries)
}

pub fn default_profile_settings(profile_name: &str) -> ProfileSettings {
    let home = dirs::home_dir()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_default();
    ProfileSettings {
        repo: RepoUrls {
            ssh: "ssh://git@ssh.example.net:7999/code/repo.git".to_string(),
            https: "https://example.net/scm/code/repo.git".to_string(),
        },
        paths: ProfilePaths {
            dev: format!("{home}/git-projects/{profile_name}/dev/branches"),
            pr: format!("{home}/git-projects/{profile_name}/pr/branches"),
        },
        files: vec![FileMapping {
            source: "files/.test-env".to_string(),
            dest: ".env".to_string(),
        }],
        pre_clone: Vec::new(),
        post_clone: Vec::new(),
    }
}

pub fn write_profile_settings(profile_dir: &Path, settings: &ProfileSettings) -> Result<()> {
    std::fs::create_dir_all(profile_dir)?;
    let path = profile_dir.join("settings.json");
    let json = serde_json::to_string_pretty(settings)?;
    std::fs::write(&path, format!("{json}\n"))
        .with_context(|| format!("writing {}", path.display()))
}

pub fn diff_profiles(a: &ProfileSettings, b: &ProfileSettings) -> Vec<DiffEntry> {
    let a_json = serde_json::to_value(a).unwrap_or_default();
    let b_json = serde_json::to_value(b).unwrap_or_default();
    diff_values("", &a_json, &b_json)
}

fn diff_values(prefix: &str, a: &serde_json::Value, b: &serde_json::Value) -> Vec<DiffEntry> {
    use serde_json::Value;
    let mut out = Vec::new();
    match (a, b) {
        (Value::Object(am), Value::Object(bm)) => {
            let mut keys: Vec<&String> = am.keys().chain(bm.keys()).collect();
            keys.sort();
            keys.dedup();
            for k in keys {
                let key = if prefix.is_empty() {
                    k.clone()
                } else {
                    format!("{prefix}.{k}")
                };
                out.extend(diff_values(&key, am.get(k).unwrap_or(&Value::Null), bm.get(k).unwrap_or(&Value::Null)));
            }
        }
        (va, vb) if va != vb => {
            out.push(DiffEntry {
                key: prefix.to_string(),
                value_a: Some(va.to_string()),
                value_b: Some(vb.to_string()),
            });
        }
        _ => {}
    }
    out
}
