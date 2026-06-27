use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct GitRc {
    pub profiles_dir: String,
    pub default_profile: Option<String>,
}

impl Default for GitRc {
    fn default() -> Self {
        Self {
            profiles_dir: "~/.gitrc-profiles".to_string(),
            default_profile: None,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RepoUrls {
    pub ssh: String,
    pub https: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ProfilePaths {
    pub dev: String,
    pub pr: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FileMapping {
    pub source: String,
    pub dest: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct ProfileSettings {
    pub repo: RepoUrls,
    pub paths: ProfilePaths,
    #[serde(default)]
    pub files: Vec<FileMapping>,
    #[serde(default)]
    pub pre_clone: Vec<String>,
    #[serde(default)]
    pub post_clone: Vec<String>,
}
