use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, ListState},
    Frame,
};

use crate::app::App;

const ITEMS: &[&str] = &[
    "  Profiles",
    "  Branches",
    "  Clone",
    "  Audit Log",
    "  Config",
];

pub fn render(f: &mut Frame, app: &App, area: Rect, selected: usize) {
    let items: Vec<ListItem> = ITEMS
        .iter()
        .map(|&label| ListItem::new(Line::from(label)))
        .collect();

    let mut state = ListState::default();
    state.select(Some(selected));

    let list = List::new(items)
        .block(
            Block::default()
                .title(" git-brancher ")
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

    f.render_stateful_widget(list, area, &mut state);
}
