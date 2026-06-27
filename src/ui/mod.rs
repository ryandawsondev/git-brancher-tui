use ratatui::{
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::{app::App, state::screen::Screen};

mod layout;
mod screens;
pub mod widgets;

use layout::app_layout;
use widgets::status_bar::render_status_bar;

pub fn render(f: &mut Frame, app: &App) {
    let layout = app_layout(f.area());

    render_header(f, app, layout.header);
    render_body(f, app, layout.body);
    render_status_bar(f, app, layout.status);
}

fn render_header(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    let title = format!(" git-brancher  {} ", app.screen.name());
    let header = Paragraph::new(Line::from(vec![
        Span::styled("git-brancher", Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)),
        Span::raw("  "),
        Span::styled(app.screen.name(), Style::default().fg(Color::White)),
    ]))
    .block(Block::default().borders(Borders::BOTTOM));
    f.render_widget(header, area);
}

fn render_body(f: &mut Frame, app: &App, area: ratatui::layout::Rect) {
    match &app.screen {
        Screen::Dashboard { selected } => {
            screens::dashboard::render(f, app, area, *selected);
        }
        Screen::ProfileList { .. } => screens::stub::render(f, "Profiles", area),
        Screen::ProfileDetail { name } => screens::stub::render(f, &format!("Profile: {name}"), area),
        Screen::ProfileCreate(_) => screens::stub::render(f, "Create Profile", area),
        Screen::ProfileEdit { name, .. } => screens::stub::render(f, &format!("Edit: {name}"), area),
        Screen::ProfileDiff { name_a, name_b, .. } => {
            screens::stub::render(f, &format!("Diff: {name_a} vs {name_b}"), area)
        }
        Screen::ProfileDoctor { name, .. } => screens::stub::render(f, &format!("Doctor: {name}"), area),
        Screen::BranchList { profile, .. } => screens::stub::render(f, &format!("Branches: {profile}"), area),
        Screen::BranchStatus { profile, .. } => screens::stub::render(f, &format!("Status: {profile}"), area),
        Screen::BranchClean { .. } => screens::stub::render(f, "Clean Branches", area),
        Screen::BranchDiff { branch, .. } => screens::stub::render(f, &format!("Diff: {branch}"), area),
        Screen::Clone(_) => screens::stub::render(f, "Clone Wizard", area),
        Screen::AuditLog { .. } => screens::stub::render(f, "Audit Log", area),
        Screen::Config { .. } => screens::stub::render(f, "Config", area),
        Screen::FirstRun(_) => screens::stub::render(f, "Setup", area),
    }
}
