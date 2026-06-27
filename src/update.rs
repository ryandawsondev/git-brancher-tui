use std::path::Path;

use crate::{
    actions::{Action, TaskOutput},
    app::{App, AsyncTask},
    ops,
    state::{
        config::{GitRc, ProfileSettings, ProfilePaths, RepoUrls},
        data::MsgLevel,
        screen::{FormState, Screen},
    },
    util::paths::get_profile_dir,
};

pub fn update(app: &mut App, action: Action) {
    match action {
        Action::Quit => app.should_quit = true,

        Action::NavigateTo(screen) => {
            app.navigate_to(screen);
            maybe_spawn_load_task(app);
        }

        Action::NavigateBack => {
            if app.prev_screens.is_empty() {
                app.should_quit = true;
            } else {
                app.navigate_back();
                maybe_spawn_load_task(app);
            }
        }

        Action::HelpToggle => app.show_help = !app.show_help,
        Action::ClearStatus => app.clear_status(),
        Action::StatusMessage(msg, level) => app.set_status(msg, level),

        Action::SelectUp => adjust_selection(app, -1),
        Action::SelectDown => adjust_selection(app, 1),
        Action::SelectToggle => toggle_selection(app),

        Action::Confirm => handle_confirm(app),
        Action::New => handle_new(app),
        Action::Delete => handle_delete(app),
        Action::Edit => handle_edit(app),

        Action::TabNext => handle_tab(app, true),
        Action::TabPrev => handle_tab(app, false),

        Action::InputChar(c) => handle_input_char(app, c),
        Action::InputBackspace => handle_input_backspace(app),

        Action::TaskComplete(output) => handle_task_complete(app, output),

        Action::Cancel | Action::Refresh => maybe_spawn_load_task(app),
    }
}

// --- selection ---

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
        Screen::ProfileDetail { action_selected, .. } => {
            use crate::ui::screens::profile_detail::ACTIONS;
            *action_selected =
                (*action_selected as i64 + delta).rem_euclid(ACTIONS.len() as i64) as usize;
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
        // Phase 3
    }
}

// --- confirm ---

fn handle_confirm(app: &mut App) {
    match app.screen.clone() {
        Screen::Dashboard { selected } => handle_dashboard_confirm(app, selected),
        Screen::ProfileList { selected } => handle_profile_list_confirm(app, selected),
        Screen::ProfileDetail { name, action_selected } => {
            handle_profile_detail_confirm(app, name, action_selected)
        }
        Screen::ProfileCreate(form) => handle_profile_create_submit(app, form),
        Screen::ProfileEdit { name, form } => handle_profile_edit_submit(app, name, form),
        Screen::FirstRun(form) => handle_first_run_submit(app, form),
        _ => {}
    }
}

fn handle_dashboard_confirm(app: &mut App, selected: usize) {
    let screen = match selected {
        0 => Screen::ProfileList { selected: 0 },
        1 => {
            if app.profiles.is_empty() {
                app.set_status("No profiles — create one first (Profiles → n)".to_string(), MsgLevel::Warn);
                return;
            }
            Screen::ProfileList { selected: 0 }
        }
        2 => Screen::Clone(Default::default()),
        3 => {
            let entries = app.audit_log.clone();
            Screen::AuditLog { entries, offset: 0 }
        }
        4 => Screen::Config { raw: app.config.clone().unwrap_or_default() },
        _ => return,
    };
    app.navigate_to(screen);
    maybe_spawn_load_task(app);
}

fn handle_profile_list_confirm(app: &mut App, selected: usize) {
    if let Some(profile) = app.profiles.get(selected) {
        let name = profile.name.clone();
        app.navigate_to(Screen::ProfileDetail { name, action_selected: 0 });
    }
}

fn handle_profile_detail_confirm(app: &mut App, profile_name: String, action: usize) {
    use crate::ui::screens::profile_detail::ACTIONS;
    match action {
        0 => {
            // View Branches → Phase 3
            app.navigate_to(Screen::BranchList {
                profile: profile_name,
                mode: crate::state::data::BranchMode::Both,
                selected: 0,
            });
        }
        1 => handle_edit_for_profile(app, &profile_name),
        2 => {
            // Run Doctor
            let config = match app.config.clone() {
                Some(c) => c,
                None => {
                    app.set_status("No config loaded".to_string(), MsgLevel::Error);
                    return;
                }
            };
            app.navigate_to(Screen::ProfileDoctor {
                name: profile_name.clone(),
                results: Vec::new(),
            });
            spawn_doctor_task(app, profile_name, config);
        }
        3 => {
            app.set_status("Profile diff — Phase 3".to_string(), MsgLevel::Info);
        }
        _ => {
            app.set_status(
                format!("'{}' — coming soon", ACTIONS.get(action).unwrap_or(&"?")),
                MsgLevel::Info,
            );
        }
    }
}

fn handle_edit_for_profile(app: &mut App, profile_name: &str) {
    let config = match &app.config {
        Some(c) => c,
        None => {
            app.set_status("No config loaded".to_string(), MsgLevel::Error);
            return;
        }
    };
    let profile = app.profiles.iter().find(|p| p.name == profile_name);
    let settings = profile.and_then(|p| p.settings.as_ref());

    let mut form = FormState::new(vec![
        ("SSH URL", "ssh://git@host:7999/code/repo.git"),
        ("HTTPS URL", "https://host/scm/code/repo.git"),
        ("Dev Path", "~/git-projects/NAME/dev/branches"),
        ("PR Path", "~/git-projects/NAME/pr/branches"),
    ]);

    if let Some(s) = settings {
        if let Some(f) = form.fields.get_mut(0) { f.value = s.repo.ssh.clone(); }
        if let Some(f) = form.fields.get_mut(1) { f.value = s.repo.https.clone(); }
        if let Some(f) = form.fields.get_mut(2) { f.value = s.paths.dev.clone(); }
        if let Some(f) = form.fields.get_mut(3) { f.value = s.paths.pr.clone(); }
    }

    app.navigate_to(Screen::ProfileEdit {
        name: profile_name.to_string(),
        form,
    });
}

fn handle_profile_create_submit(app: &mut App, form: FormState) {
    let get = |i: usize| form.fields.get(i).map(|f| f.value.trim().to_string()).unwrap_or_default();
    let name = get(0);
    let ssh = get(1);
    let https = get(2);
    let dev = get(3);
    let pr = get(4);

    if name.is_empty() {
        app.set_status("Name is required".to_string(), MsgLevel::Warn);
        return;
    }
    let config = match app.config.clone() {
        Some(c) => c,
        None => {
            app.set_status("No config loaded".to_string(), MsgLevel::Error);
            return;
        }
    };

    let profile_dir = get_profile_dir(&config, &name);
    let settings = ProfileSettings {
        repo: RepoUrls {
            ssh: if ssh.is_empty() { format!("ssh://git@host:7999/{name}/repo.git") } else { ssh },
            https: if https.is_empty() { format!("https://host/scm/{name}/repo.git") } else { https },
        },
        paths: ProfilePaths {
            dev: if dev.is_empty() { format!("~/git-projects/{name}/dev/branches") } else { dev },
            pr: if pr.is_empty() { format!("~/git-projects/{name}/pr/branches") } else { pr },
        },
        files: Vec::new(),
        pre_clone: Vec::new(),
        post_clone: Vec::new(),
    };

    match ops::profile::write_profile_settings(Path::new(&profile_dir), &settings) {
        Ok(()) => {
            app.set_status(format!("Created profile '{name}'"), MsgLevel::Info);
            app.navigate_back();
            spawn_profile_load_task(app);
        }
        Err(e) => app.set_status(format!("Error: {e}"), MsgLevel::Error),
    }
}

fn handle_profile_edit_submit(app: &mut App, name: String, form: FormState) {
    let get = |i: usize| form.fields.get(i).map(|f| f.value.trim().to_string()).unwrap_or_default();

    let config = match app.config.clone() {
        Some(c) => c,
        None => {
            app.set_status("No config loaded".to_string(), MsgLevel::Error);
            return;
        }
    };

    let profile_dir = get_profile_dir(&config, &name);
    let existing = app.profiles.iter().find(|p| p.name == name)
        .and_then(|p| p.settings.clone());

    let settings = ProfileSettings {
        repo: RepoUrls { ssh: get(0), https: get(1) },
        paths: ProfilePaths { dev: get(2), pr: get(3) },
        files: existing.as_ref().map(|s| s.files.clone()).unwrap_or_default(),
        pre_clone: existing.as_ref().map(|s| s.pre_clone.clone()).unwrap_or_default(),
        post_clone: existing.as_ref().map(|s| s.post_clone.clone()).unwrap_or_default(),
    };

    match ops::profile::write_profile_settings(Path::new(&profile_dir), &settings) {
        Ok(()) => {
            app.set_status(format!("Saved '{name}'"), MsgLevel::Info);
            app.navigate_back();
            spawn_profile_load_task(app);
        }
        Err(e) => app.set_status(format!("Error: {e}"), MsgLevel::Error),
    }
}

fn handle_first_run_submit(app: &mut App, form: FormState) {
    let profiles_dir = form.fields.get(0).map(|f| f.value.trim().to_string()).unwrap_or_default();
    let default_profile = form.fields.get(1)
        .map(|f| f.value.trim().to_string())
        .filter(|v| !v.is_empty());

    if profiles_dir.is_empty() {
        app.set_status("Profiles directory is required".to_string(), MsgLevel::Warn);
        return;
    }

    let config = GitRc { profiles_dir, default_profile };

    match ops::config::write_gitrc(&config) {
        Ok(()) => {
            app.config = Some(config);
            app.set_status("Config saved. Welcome!".to_string(), MsgLevel::Info);
            // replace FirstRun with Dashboard
            app.screen = Screen::Dashboard { selected: 0 };
            app.prev_screens.clear();
        }
        Err(e) => app.set_status(format!("Error: {e}"), MsgLevel::Error),
    }
}

// --- new / delete / edit ---

fn handle_new(app: &mut App) {
    match &app.screen {
        Screen::ProfileList { .. } | Screen::Dashboard { .. } => {
            let form = FormState::new(vec![
                ("Name", "my-profile"),
                ("SSH URL", "ssh://git@host:7999/code/repo.git"),
                ("HTTPS URL", "https://host/scm/code/repo.git"),
                ("Dev Path", "~/git-projects/NAME/dev/branches"),
                ("PR Path", "~/git-projects/NAME/pr/branches"),
            ]);
            app.navigate_to(Screen::ProfileCreate(form));
        }
        _ => {}
    }
}

fn handle_delete(app: &mut App) {
    let (selected, config) = match (&app.screen, &app.config) {
        (Screen::ProfileList { selected }, Some(c)) => (*selected, c.clone()),
        _ => return,
    };

    if let Some(profile) = app.profiles.get(selected) {
        let profile_dir = get_profile_dir(&config, &profile.name);
        let name = profile.name.clone();
        match std::fs::remove_dir_all(&profile_dir) {
            Ok(()) => {
                app.set_status(format!("Deleted profile '{name}'"), MsgLevel::Info);
                spawn_profile_load_task(app);
            }
            Err(e) => app.set_status(format!("Delete failed: {e}"), MsgLevel::Error),
        }
    }
}

fn handle_edit(app: &mut App) {
    if let Screen::ProfileDetail { name, .. } = app.screen.clone() {
        handle_edit_for_profile(app, &name);
    }
}

// --- form helpers ---

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
            if let Some(v) = form.focused_value_mut() { v.push(c); }
        }
        Screen::ProfileEdit { form, .. } => {
            if let Some(v) = form.focused_value_mut() { v.push(c); }
        }
        _ => {}
    }
}

fn handle_input_backspace(app: &mut App) {
    match &mut app.screen {
        Screen::ProfileCreate(form) | Screen::FirstRun(form) => {
            if let Some(v) = form.focused_value_mut() { v.pop(); }
        }
        Screen::ProfileEdit { form, .. } => {
            if let Some(v) = form.focused_value_mut() { v.pop(); }
        }
        _ => {}
    }
}

// --- task completion ---

fn handle_task_complete(app: &mut App, output: TaskOutput) {
    match output {
        TaskOutput::ProfileList(profiles) => {
            app.profiles = profiles;
            if let Screen::ProfileList { selected } = &mut app.screen {
                if *selected >= app.profiles.len() && !app.profiles.is_empty() {
                    *selected = app.profiles.len() - 1;
                }
            }
        }
        TaskOutput::BranchList(branches) => app.branches = branches,
        TaskOutput::AuditLog(entries) => {
            if let Screen::AuditLog { entries: ref mut e, .. } = app.screen {
                *e = entries.clone();
            }
            app.audit_log = entries;
        }
        TaskOutput::DoctorResults(results) => {
            if let Screen::ProfileDoctor { results: ref mut r, .. } = app.screen {
                *r = results;
            }
        }
        TaskOutput::CommandOutput(out) => app.set_status(out, MsgLevel::Info),
        _ => {}
    }
}

// --- async task spawning ---

fn maybe_spawn_load_task(app: &mut App) {
    match &app.screen {
        Screen::ProfileList { .. } => spawn_profile_load_task(app),
        Screen::AuditLog { .. } => spawn_audit_load_task(app),
        _ => {}
    }
}

fn spawn_profile_load_task(app: &mut App) {
    let config = match app.config.clone() {
        Some(c) => c,
        None => return,
    };
    let handle = tokio::spawn(async move {
        let profiles = ops::profile::list_profiles(&config)?;
        Ok::<_, anyhow::Error>(TaskOutput::ProfileList(profiles))
    });
    app.async_task = Some(AsyncTask {
        label: "Loading profiles…".to_string(),
        handle,
    });
}

fn spawn_audit_load_task(app: &mut App) {
    let handle = tokio::spawn(async move {
        let entries = ops::audit::read_audit_log()?;
        Ok::<_, anyhow::Error>(TaskOutput::AuditLog(entries))
    });
    app.async_task = Some(AsyncTask {
        label: "Loading audit log…".to_string(),
        handle,
    });
}

fn spawn_doctor_task(app: &mut App, profile_name: String, config: GitRc) {
    let label_name = profile_name.clone();
    let handle = tokio::spawn(async move {
        let profile_dir = get_profile_dir(&config, &profile_name);
        let settings_path = format!("{profile_dir}/settings.json");
        let raw = std::fs::read_to_string(&settings_path)
            .map_err(|e| anyhow::anyhow!("reading settings.json: {e}"))?;
        let settings: ProfileSettings = serde_json::from_str(&raw)
            .map_err(|e| anyhow::anyhow!("parsing settings.json: {e}"))?;
        let results = ops::doctor::run_all_checks(settings, profile_dir).await;
        Ok::<_, anyhow::Error>(TaskOutput::DoctorResults(results))
    });
    app.async_task = Some(AsyncTask {
        label: format!("Running doctor for {label_name}…"),
        handle,
    });
}
