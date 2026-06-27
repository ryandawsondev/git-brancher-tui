use crate::{
    actions::TaskOutput,
    state::{
        config::GitRc,
        data::{AuditEntry, BranchEntry, MsgLevel, ProfileSummary},
        screen::Screen,
    },
};

pub struct AsyncTask {
    pub label: String,
    pub handle: tokio::task::JoinHandle<anyhow::Result<TaskOutput>>,
}

pub struct App {
    pub screen: Screen,
    pub prev_screens: Vec<Screen>,
    pub config: Option<GitRc>,
    pub profiles: Vec<ProfileSummary>,
    pub branches: Vec<BranchEntry>,
    pub audit_log: Vec<AuditEntry>,
    pub async_task: Option<AsyncTask>,
    pub status_msg: Option<(String, MsgLevel)>,
    pub should_quit: bool,
    pub show_help: bool,
}

impl App {
    pub fn new() -> Self {
        Self {
            screen: Screen::Dashboard { selected: 0 },
            prev_screens: Vec::new(),
            config: None,
            profiles: Vec::new(),
            branches: Vec::new(),
            audit_log: Vec::new(),
            async_task: None,
            status_msg: None,
            should_quit: false,
            show_help: false,
        }
    }

    pub fn navigate_to(&mut self, screen: Screen) {
        let prev = std::mem::replace(&mut self.screen, screen);
        self.prev_screens.push(prev);
    }

    pub fn navigate_back(&mut self) {
        if let Some(prev) = self.prev_screens.pop() {
            self.screen = prev;
        }
    }

    pub fn set_status(&mut self, msg: impl Into<String>, level: MsgLevel) {
        self.status_msg = Some((msg.into(), level));
    }

    pub fn clear_status(&mut self) {
        self.status_msg = None;
    }
}
