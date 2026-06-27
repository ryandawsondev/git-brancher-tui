use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::state::config::GitRc;

pub fn render(f: &mut Frame, raw: &GitRc, area: Rect) {
    let block = Block::default()
        .title(" Config (~/.gitrc) ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(inner);

    let key_style = Style::default().fg(Color::Yellow);

    let lines = vec![
        Line::from(vec![
            Span::styled("profilesDir    ", key_style),
            Span::raw(raw.profiles_dir.clone()),
        ]),
        Line::from(vec![
            Span::styled("defaultProfile ", key_style),
            Span::raw(
                raw.default_profile
                    .as_deref()
                    .unwrap_or("(none)")
                    .to_string(),
            ),
        ]),
    ];

    f.render_widget(Paragraph::new(lines), chunks[0]);
    f.render_widget(
        Paragraph::new("Esc=back").style(Style::default().fg(Color::DarkGray)),
        chunks[1],
    );
}
