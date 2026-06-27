use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

use crate::state::screen::FormState;
use crate::ui::widgets::form::render_form;

pub fn render(f: &mut Frame, form: &FormState, area: Rect) {
    let block = Block::default()
        .title(" git-brancher — First-Time Setup ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Green));

    let inner = block.inner(area);
    f.render_widget(block, area);

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Length(2), Constraint::Min(0)])
        .split(inner);

    f.render_widget(
        Paragraph::new("No ~/.gitrc found. Configure where your profiles live."),
        chunks[0],
    );

    render_form(f, form, chunks[1]);
}
