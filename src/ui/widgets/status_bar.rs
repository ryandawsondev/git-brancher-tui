use ratatui::{
    layout::Rect,
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

use crate::{app::App, state::data::MsgLevel};

pub fn render_status_bar(f: &mut Frame, app: &App, area: Rect) {
    let (text, style) = if let Some((msg, level)) = &app.status_msg {
        let color = match level {
            MsgLevel::Info => Color::Cyan,
            MsgLevel::Warn => Color::Yellow,
            MsgLevel::Error => Color::Red,
        };
        (msg.as_str(), Style::default().fg(color))
    } else {
        ("q quit  Esc back  ? help", Style::default().fg(Color::DarkGray))
    };
    let bar = Paragraph::new(Line::from(vec![Span::styled(text, style)]));
    f.render_widget(bar, area);
}
