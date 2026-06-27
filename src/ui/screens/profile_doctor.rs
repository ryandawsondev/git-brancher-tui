use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph},
    Frame,
};

use crate::state::data::CheckResult;

pub fn render(f: &mut Frame, area: Rect, name: &str, results: &[CheckResult]) {
    let block = Block::default()
        .title(format!(" Doctor: {name} "))
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Magenta));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if results.is_empty() {
        f.render_widget(
            Paragraph::new("Running checks...").style(Style::default().fg(Color::Yellow)),
            inner,
        );
        return;
    }

    let pass_count = results.iter().filter(|r| r.ok).count();
    let total = results.len();

    let header_color = if pass_count == total { Color::Green } else { Color::Red };
    let summary = format!(" {pass_count}/{total} checks passed ");

    let items: Vec<ListItem> = std::iter::once(
        ListItem::new(Line::from(Span::styled(summary, Style::default().fg(header_color)))),
    )
    .chain(std::iter::once(ListItem::new("")))
    .chain(results.iter().map(|r| {
        let (icon, color) = if r.ok { ("✓", Color::Green) } else { ("✗", Color::Red) };
        ListItem::new(Line::from(vec![
            Span::styled(format!("{icon} "), Style::default().fg(color)),
            Span::raw(r.label.clone()),
        ]))
    }))
    .collect();

    f.render_widget(List::new(items), inner);
}
