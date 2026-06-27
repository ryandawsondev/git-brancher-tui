use ratatui::{
    layout::Rect,
    style::{Color, Style},
    widgets::{Block, BorderType, Borders, Paragraph},
    Frame,
};

pub fn render(f: &mut Frame, title: &str, area: Rect) {
    let block = Block::default()
        .title(format!(" {title} "))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::DarkGray));
    let text = Paragraph::new("Coming in Phase 2+  (Esc to go back)").block(block);
    f.render_widget(text, area);
}
