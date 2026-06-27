use crate::state::{
    data::{AuditEntry, BranchEntry, BranchStatusRow, CheckResult, DiffEntry, MsgLevel, ProfileSummary},
    screen::Screen,
};

pub enum Action {
    Quit,
    NavigateTo(Screen),
    NavigateBack,
    SelectUp,
    SelectDown,
    SelectToggle,
    Confirm,
    Cancel,
    TabNext,
    TabPrev,
    Refresh,
    HelpToggle,
    InputChar(char),
    InputBackspace,
    TaskComplete(TaskOutput),
    StatusMessage(String, MsgLevel),
    ClearStatus,
}

pub enum TaskOutput {
    BranchList(Vec<BranchEntry>),
    StatusRows(Vec<BranchStatusRow>),
    CloneDone { branch: String },
    AuditLog(Vec<AuditEntry>),
    ProfileList(Vec<ProfileSummary>),
    DoctorResults(Vec<CheckResult>),
    ProfileDiff(Vec<DiffEntry>),
    CommandOutput(String),
}
