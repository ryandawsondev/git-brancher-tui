use crate::{
    actions::{Action, TaskOutput},
    app::App,
    state::{
        data::MsgLevel,
        screen::Screen,
    },
};

pub fn update(app: &mut App, action: Action) {
    match action {
        Action::Quit => app.should_quit = true,

        Action::NavigateTo(screen) => app.navigate_to(screen),

        Action::NavigateBack => {
            if app.prev_screens.is_empty() {
                app.should_quit = true;
            } else {
                app.navigate_back();
            }
        }

        Action::HelpToggle => app.show_help = !app.show_help,

        Action::ClearStatus => app.clear_status(),

        Action::StatusMessage(msg, level) => app.set_status(msg, level),

        Action::SelectUp => adjust_selection(app, -1),

        Action::SelectDown => adjust_selection(app, 1),

        Action::SelectToggle => toggle_selection(app),

        Action::Confirm => handle_confirm(app),

        Action::TabNext => handle_tab(app, true),

        Action::TabPrev => handle_tab(app, false),

        Action::InputChar(c) => handle_input_char(app, c),

        Action::InputBackspace => handle_input_backspace(app),

        Action::TaskComplete(output) => handle_task_complete(app, output),

        Action::Cancel | Action::Refresh => {}
    }
}

fn adjust_selection(app: &mut App, delta: i64) {
    match &mut app.screen {
        Screen::Dashboard { selected } => {
            const ITEMS: usize = 5;
            *selected = (*selected as i64 + delta).rem_euclid(ITEMS as i64) as usize;
        }
        Screen::ProfileList { selected } => {
            let len = app.profiles.len();
            if len > 0 {
                *selected = (*selected as i64 + delta).rem_euclid(len as i64) as usize;
            }
        }
        Screen::BranchList { selected, .. } => {
            let len = app.branches.len();
            if len > 0 {
                *selected = (*selected as i64 + delta).rem_euclid(len as i64) as usize;
            }
        }
        Screen::AuditLog { offset, entries } => {
            let len = entries.len();
            if delta > 0 && *offset + 1 < len {
                *offset += 1;
            } else if delta < 0 && *offset > 0 {
                *offset -= 1;
            }
        }
        _ => {}
    }
}

fn toggle_selection(app: &mut App) {
    if let Screen::BranchClean { entries: _, selected: _ } = &mut app.screen {
        // multi-select toggle implemented in Phase 3
    }
}

fn handle_confirm(app: &mut App) {
    match &app.screen {
        Screen::Dashboard { selected } => {
            let idx = *selected;
            let screen = match idx {
                0 => Screen::ProfileList { selected: 0 },
                1 => {
                    if app.profiles.is_empty() {
                        app.set_status("No profiles — create one first".to_string(), MsgLevel::Warn);
                        return;
                    }
                    Screen::ProfileList { selected: 0 }
                }
                2 => Screen::Clone(Default::default()),
                3 => Screen::AuditLog { entries: app.audit_log.clone(), offset: 0 },
                4 => Screen::Config {
                    raw: app.config.clone().unwrap_or_default(),
                },
                _ => return,
            };
            app.navigate_to(screen);
        }
        Screen::ProfileList { selected } => {
            let idx = *selected;
            if let Some(profile) = app.profiles.get(idx) {
                let name = profile.name.clone();
                app.navigate_to(Screen::ProfileDetail { name });
            }
        }
        _ => {}
    }
}

fn handle_tab(app: &mut App, forward: bool) {
    match &mut app.screen {
        Screen::ProfileCreate(form) | Screen::FirstRun(form) => {
            if forward { form.focus_next() } else { form.focus_prev() }
        }
        Screen::ProfileEdit { form, .. } => {
            if forward { form.focus_next() } else { form.focus_prev() }
        }
        _ => {}
    }
}

fn handle_input_char(app: &mut App, c: char) {
    match &mut app.screen {
        Screen::ProfileCreate(form) | Screen::FirstRun(form) => {
            if let Some(v) = form.focused_value_mut() {
                v.push(c);
            }
        }
        Screen::ProfileEdit { form, .. } => {
            if let Some(v) = form.focused_value_mut() {
                v.push(c);
            }
        }
        _ => {}
    }
}

fn handle_input_backspace(app: &mut App) {
    match &mut app.screen {
        Screen::ProfileCreate(form) | Screen::FirstRun(form) => {
            if let Some(v) = form.focused_value_mut() {
                v.pop();
            }
        }
        Screen::ProfileEdit { form, .. } => {
            if let Some(v) = form.focused_value_mut() {
                v.pop();
            }
        }
        _ => {}
    }
}

fn handle_task_complete(app: &mut App, output: TaskOutput) {
    match output {
        TaskOutput::ProfileList(profiles) => app.profiles = profiles,
        TaskOutput::BranchList(branches) => app.branches = branches,
        TaskOutput::AuditLog(entries) => app.audit_log = entries,
        TaskOutput::CommandOutput(out) => {
            app.set_status(out, MsgLevel::Info);
        }
        _ => {}
    }
}
