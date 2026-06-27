use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::Line,
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::App;

pub const ACTIONS: &[&str] = &[
    "View Branches",
    "Edit Settings",
    "Run Doctor",
    "Diff with Profile",
    "Clone Branch",
    "Export",
    "Import",
    "Copy",
    "Rename",
    "Delete Profile",
];

pub fn render(f: &mut Frame, app: &App, area: Rect, name: &str, action_selected: usize) {
    let chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(65), Constraint::Percentage(35)])
        .split(area);

    render_settings(f, app, chunks[0], name);
    render_actions(f, chunks[1], action_selected);
}

fn render_settings(f: &mut Frame, app: &App, area: Rect, name: &str) {
    let profile = app.profiles.iter().find(|p| p.name == name);

    let lines: Vec<Line> = match profile.and_then(|p| p.settings.as_ref()) {
        Some(s) => vec![
            Line::from(format!("SSH    {}", s.repo.ssh)),
            Line::from(format!("HTTPS  {}", s.repo.https)),
            Line::from(""),
            Line::from(format!("Dev    {}", s.paths.dev)),
            Line::from(format!("PR     {}", s.paths.pr)),
            Line::from(""),
            Line::from(format!("Files  {}", s.files.len())),
            Line::from(format!("Pre    {} hooks", s.pre_clone.len())),
            Line::from(format!("Post   {} hooks", s.post_clone.len())),
        ],
        None => vec![Line::raw("No settings.json found")],
    };

    f.render_widget(
        Paragraph::new(lines).block(
            Block::default()
                .title(format!(" {name} "))
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)),
        ),
        area,
    );
}

fn render_actions(f: &mut Frame, area: Rect, selected: usize) {
    let items: Vec<ListItem> = ACTIONS.iter().map(|a| ListItem::new(*a)).collect();
    let mut state = ListState::default();
    state.select(Some(selected));

    f.render_stateful_widget(
        List::new(items)
            .block(
                Block::default()
                    .title(" Actions ")
                    .borders(Borders::ALL)
                    .border_type(BorderType::Rounded)
                    .border_style(Style::default().fg(Color::Yellow)),
            )
            .highlight_style(
                Style::default()
                    .fg(Color::Black)
                    .bg(Color::Yellow)
                    .add_modifier(Modifier::BOLD),
            )
            .highlight_symbol("▶ "),
        area,
        &mut state,
    );
}
