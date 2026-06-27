use chrono::{DateTime, Local};
use serde::{Deserialize, Serialize};

use super::config::ProfileSettings;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchMode {
    Dev,
    Pr,
    Both,
}

impl BranchMode {
    pub fn as_str(self) -> &'static str {
        match self {
            BranchMode::Dev => "dev",
            BranchMode::Pr => "pr",
            BranchMode::Both => "both",
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum BranchSlot {
    Dev,
    Pr,
}

impl BranchSlot {
    pub fn as_str(self) -> &'static str {
        match self {
            BranchSlot::Dev => "dev",
            BranchSlot::Pr => "pr",
        }
    }
}

#[derive(Debug, Clone)]
pub struct BranchEntry {
    pub name: String,
    pub slot: BranchSlot,
    pub dir: String,
    pub last_modified: DateTime<Local>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuditEntry {
    pub ts: String,
    pub command: String,
    pub profile: String,
    pub branch: String,
    pub mode: String,
    pub proto: String,
    pub target: String,
}

#[derive(Debug, Clone)]
pub struct ProfileSummary {
    pub name: String,
    pub profile_dir: String,
    pub settings: Option<ProfileSettings>,
    pub settings_path: String,
}

#[derive(Debug, Clone)]
pub struct DiffEntry {
    pub key: String,
    pub value_a: Option<String>,
    pub value_b: Option<String>,
}

#[derive(Debug, Clone)]
pub struct CheckResult {
    pub label: String,
    pub ok: bool,
    pub detail: Option<String>,
}

#[derive(Debug, Clone)]
pub struct BranchStatusRow {
    pub name: String,
    pub slot: BranchSlot,
    pub dir: String,
    pub ahead: Option<u32>,
    pub behind: Option<u32>,
    pub dirty: bool,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum MsgLevel {
    Info,
    Warn,
    Error,
}
