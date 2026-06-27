use ratatui::{
    layout::{Constraint, Rect},
    style::{Color, Modifier, Style},
    widgets::{Block, BorderType, Borders, Cell, Paragraph, Row, Table},
    Frame,
};

use crate::state::data::AuditEntry;

pub fn render(f: &mut Frame, entries: &[AuditEntry], offset: usize, area: Rect) {
    let block = Block::default()
        .title(" Audit Log ")
        .borders(Borders::ALL)
        .border_type(BorderType::Rounded)
        .border_style(Style::default().fg(Color::Cyan));

    let inner = block.inner(area);
    f.render_widget(block, area);

    if entries.is_empty() {
        f.render_widget(
            Paragraph::new("No audit entries yet.").style(Style::default().fg(Color::DarkGray)),
            inner,
        );
        return;
    }

    let header = Row::new(["Time", "Command", "Profile", "Branch", "Mode"])
        .style(Style::default().fg(Color::Yellow).add_modifier(Modifier::BOLD));

    // Most recent first
    let visible_height = inner.height.saturating_sub(2) as usize;
    let rows: Vec<Row> = entries
        .iter()
        .rev()
        .skip(offset)
        .take(visible_height)
        .map(|e| {
            let ts = e.ts.chars().take(19).collect::<String>();
            Row::new([
                Cell::from(ts),
                Cell::from(e.command.clone()),
                Cell::from(e.profile.clone()),
                Cell::from(e.branch.clone()),
                Cell::from(e.mode.clone()),
            ])
        })
        .collect();

    let table = Table::new(
        rows,
        [
            Constraint::Length(19),
            Constraint::Length(8),
            Constraint::Length(14),
            Constraint::Min(16),
            Constraint::Length(6),
        ],
    )
    .header(header)
    .column_spacing(1);

    f.render_widget(table, inner);
}
