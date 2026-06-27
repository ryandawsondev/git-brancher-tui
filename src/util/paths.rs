use crate::state::config::GitRc;

pub fn resolve_path(p: &str) -> String {
    let home = dirs::home_dir()
        .map(|h| h.to_string_lossy().to_string())
        .unwrap_or_default();
    p.replace('~', &home).replace("%USERPROFILE%", &home)
}

pub fn resolve_profiles_dir(config: &GitRc) -> String {
    resolve_path(&config.profiles_dir)
}

pub fn get_profile_dir(config: &GitRc, profile_name: &str) -> String {
    let base = resolve_profiles_dir(config);
    format!("{base}/{profile_name}")
}
