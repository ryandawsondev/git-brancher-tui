use std::collections::BTreeSet;

use super::config::GitRc;
use super::data::{BranchEntry, BranchMode, BranchStatusRow, AuditEntry, DiffEntry, CheckResult};

#[derive(Debug, Clone)]
pub struct FormField {
    pub label: &'static str,
    pub value: String,
    pub placeholder: &'static str,
}

#[derive(Debug, Clone, Default)]
pub struct FormState {
    pub fields: Vec<FormField>,
    pub focused: usize,
}

impl FormState {
    pub fn new(fields: Vec<(&'static str, &'static str)>) -> Self {
        Self {
            fields: fields
                .into_iter()
                .map(|(label, placeholder)| FormField {
                    label,
                    value: String::new(),
                    placeholder,
                })
                .collect(),
            focused: 0,
        }
    }

    pub fn focused_value_mut(&mut self) -> Option<&mut String> {
        self.fields.get_mut(self.focused).map(|f| &mut f.value)
    }

    pub fn focus_next(&mut self) {
        if !self.fields.is_empty() {
            self.focused = (self.focused + 1) % self.fields.len();
        }
    }

    pub fn focus_prev(&mut self) {
        if !self.fields.is_empty() {
            self.focused = self.focused.saturating_sub(1);
        }
    }
}

#[derive(Debug, Clone)]
pub struct CloneWizard {
    pub step: usize,
    pub profile: Option<String>,
    pub branch: Option<String>,
    pub mode: Option<BranchMode>,
    pub use_ssh: bool,
    pub log_lines: Vec<String>,
}

impl Default for CloneWizard {
    fn default() -> Self {
        Self {
            step: 0,
            profile: None,
            branch: None,
            mode: None,
            use_ssh: true,
            log_lines: Vec::new(),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Screen {
    Dashboard { selected: usize },
    ProfileList { selected: usize },
    ProfileDetail { name: String, action_selected: usize },
    ProfileCreate(FormState),
    ProfileEdit { name: String, form: FormState },
    ProfileDiff { name_a: String, name_b: String, diff: Vec<DiffEntry> },
    ProfileDoctor { name: String, results: Vec<CheckResult> },
    BranchList { profile: String, mode: BranchMode, selected: usize },
    BranchStatus { profile: String, mode: BranchMode, rows: Vec<BranchStatusRow> },
    BranchClean { entries: Vec<BranchEntry>, selected: BTreeSet<usize> },
    BranchDiff { profile: String, branch: String, output: String },
    Clone(CloneWizard),
    AuditLog { entries: Vec<AuditEntry>, offset: usize },
    Config { raw: GitRc },
    FirstRun(FormState),
}

impl Screen {
    pub fn name(&self) -> &'static str {
        match self {
            Screen::Dashboard { .. } => "Dashboard",
            Screen::ProfileList { .. } => "Profiles",
            Screen::ProfileDetail { .. } => "Profile Detail",
            Screen::ProfileCreate(_) => "Create Profile",
            Screen::ProfileEdit { .. } => "Edit Profile",
            Screen::ProfileDiff { .. } => "Profile Diff",
            Screen::ProfileDoctor { .. } => "Profile Doctor",
            Screen::BranchList { .. } => "Branches",
            Screen::BranchStatus { .. } => "Branch Status",
            Screen::BranchClean { .. } => "Clean Branches",
            Screen::BranchDiff { .. } => "Branch Diff",
            Screen::Clone(_) => "Clone",
            Screen::AuditLog { .. } => "Audit Log",
            Screen::Config { .. } => "Config",
            Screen::FirstRun(_) => "Setup",
        }
    }
}
