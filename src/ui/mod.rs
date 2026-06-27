use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::{app::App, state::screen::Screen};

mod layout;
pub mod screens;
pub mod widgets;

use layout::app_layout;
use widgets::status_bar::render_status_bar;

pub fn render(f: &mut Frame, app: &App) {
    let layout = app_layout(f.area());
    render_header(f, app, layout.header);
    render_body(f, app, layout.body);
    render_status_bar(f, app, layout.status);
}

fn render_header(f: &mut Frame, app: &App, area: Rect) {
    let header = Paragraph::new(Line::from(vec![
        Span::styled(
            "git-brancher",
            Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD),
        ),
        Span::raw("  "),
        Span::styled(app.screen.name(), Style::default().fg(Color::White)),
        if app.async_task.is_some() {
            Span::styled("  [loading…]", Style::default().fg(Color::Yellow))
        } else {
            Span::raw("")
        },
    ]))
    .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(header, area);
}

fn render_body(f: &mut Frame, app: &App, area: Rect) {
    match &app.screen {
        Screen::Dashboard { selected } => {
            screens::dashboard::render(f, app, area, *selected);
        }
        Screen::ProfileList { selected } => {
            screens::profile_list::render(f, app, area, *selected);
        }
        Screen::ProfileDetail { name, action_selected } => {
            screens::profile_detail::render(f, app, area, name, *action_selected);
        }
        Screen::ProfileCreate(form) => {
            screens::profile_create::render(f, form, area);
        }
        Screen::ProfileEdit { name, form } => {
            screens::profile_create::render(f, form, area);
        }
        Screen::ProfileDiff { name_a, name_b, diff } => {
            screens::stub::render(f, &format!("Diff: {name_a} vs {name_b} — Phase 3"), area);
        }
        Screen::ProfileDoctor { name, results } => {
            screens::profile_doctor::render(f, area, name, results);
        }
        Screen::BranchList { profile, .. } => {
            screens::stub::render(f, &format!("Branches: {profile} — Phase 3"), area);
        }
        Screen::BranchStatus { profile, .. } => {
            screens::stub::render(f, &format!("Status: {profile} — Phase 3"), area);
        }
        Screen::BranchClean { .. } => screens::stub::render(f, "Clean Branches — Phase 3", area),
        Screen::BranchDiff { branch, .. } => {
            screens::stub::render(f, &format!("Diff: {branch} — Phase 3"), area);
        }
        Screen::Clone(_) => screens::stub::render(f, "Clone Wizard — Phase 4", area),
        Screen::AuditLog { entries, offset } => {
            screens::audit_log::render(f, entries, *offset, area);
        }
        Screen::Config { raw } => {
            screens::config_screen::render(f, raw, area);
        }
        Screen::FirstRun(form) => {
            screens::first_run::render(f, form, area);
        }
    }
}
