use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders},
    Frame,
};

use crate::state::screen::FormState;
use crate::ui::widgets::form::render_form;

pub fn render(f: &mut Frame, form: &FormState, area: Rect) {
    let block = Block::default()
        .title(" Create Profile ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Green));

    let inner = block.inner(area);
    f.render_widget(block, area);
    render_form(f, form, inner);
}
