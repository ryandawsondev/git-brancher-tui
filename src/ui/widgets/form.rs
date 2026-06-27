use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

use crate::state::screen::FormState;

pub fn render_form(f: &mut Frame, form: &FormState, area: Rect) {
    let field_height = 3u16;
    let hint_height = 1u16;
    let fields_area = Rect {
        x: area.x,
        y: area.y,
        width: area.width,
        height: area.height.saturating_sub(hint_height + 1),
    };

    for (i, field) in form.fields.iter().enumerate() {
        let y = fields_area.y + (i as u16) * field_height;
        if y + field_height > fields_area.y + fields_area.height {
            break;
        }
        let field_area = Rect { x: fields_area.x, y, width: fields_area.width, height: field_height };

        let is_focused = i == form.focused;
        let border_color = if is_focused { Color::Cyan } else { Color::DarkGray };

        let display: String = if is_focused {
            format!("{}▌", field.value)
        } else if field.value.is_empty() {
            field.placeholder.to_string()
        } else {
            field.value.clone()
        };

        let text_style = if field.value.is_empty() && !is_focused {
            Style::default().fg(Color::DarkGray)
        } else {
            Style::default()
        };

        let block = Block::default()
            .title(format!(" {} ", field.label))
            .borders(Borders::ALL)
            .border_style(Style::default().fg(border_color));

        f.render_widget(
            Paragraph::new(Line::from(Span::styled(display, text_style))).block(block),
            field_area,
        );
    }

    let hint_y = area.y + area.height.saturating_sub(hint_height);
    let hint_area = Rect { x: area.x, y: hint_y, width: area.width, height: hint_height };
    f.render_widget(
        Paragraph::new("Tab=next field  Enter=submit  Esc=cancel")
            .style(Style::default().fg(Color::DarkGray)),
        hint_area,
    );
}
