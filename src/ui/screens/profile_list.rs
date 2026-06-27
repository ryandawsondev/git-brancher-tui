use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState, Paragraph},
    Frame,
};

use crate::app::App;

pub fn render(f: &mut Frame, app: &App, area: Rect, selected: usize) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(1)])
        .split(area);

    let items: Vec<ListItem> = if app.profiles.is_empty() {
        vec![ListItem::new(
            Span::styled(" No profiles — press 'n' to create one", Style::default().fg(Color::DarkGray)),
        )]
    } else {
        app.profiles
            .iter()
            .map(|p| {
                let (icon, color) = if p.settings.is_some() {
                    ("●", Color::Green)
                } else {
                    ("○", Color::Red)
                };
                ListItem::new(Line::from(vec![
                    Span::styled(format!(" {icon} "), Style::default().fg(color)),
                    Span::raw(p.name.clone()),
                ]))
            })
            .collect()
    };

    let mut state = ListState::default();
    if !app.profiles.is_empty() {
        state.select(Some(selected));
    }

    let list = List::new(items)
        .block(
            Block::default()
                .title(" Profiles ")
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .border_style(Style::default().fg(Color::Cyan)),
        )
        .highlight_style(
            Style::default()
                .fg(Color::Black)
                .bg(Color::Cyan)
                .add_modifier(Modifier::BOLD),
        )
        .highlight_symbol("▶ ");

    f.render_stateful_widget(list, chunks[0], &mut state);

    f.render_widget(
        Paragraph::new("n=new  d=delete  Enter=open  Esc=back")
            .style(Style::default().fg(Color::DarkGray)),
        chunks[1],
    );
}
