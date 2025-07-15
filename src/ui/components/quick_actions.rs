use crate::dashboard::widgets::QuickAction;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};

pub fn draw_quick_actions(f: &mut Frame, area: Rect, actions: &[QuickAction], selected_index: Option<usize>) {
    let action_items: Vec<ListItem> = actions.iter().enumerate().map(|(i, action)| {
        let style = if Some(i) == selected_index {
            Style::default().bg(Color::DarkGray)
        } else {
            Style::default()
        };

        ListItem::new(Line::from(vec![
            Span::styled(
                format!("[{}] ", action.hotkey.unwrap_or(' ')),
                Style::default().fg(Color::Green),
            ),
            Span::styled(&action.name, style.fg(Color::White)),
        ]))
    }).collect();

    let list = List::new(action_items)
        .block(Block::default()
            .title("Quick Actions")
            .borders(Borders::ALL)
        )
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_widget(list, area);
}