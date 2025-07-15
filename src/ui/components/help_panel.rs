use crate::app::state::AppState;
use crate::config::defaults::get_default_keybindings;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_help_panel(f: &mut Frame, area: Rect, app_state: &AppState) {
    // Create a centered popup
    let popup_area = centered_rect(80, 70, area);

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title("Help")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    f.render_widget(block, popup_area);

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(0),
            Constraint::Length(3),
        ])
        .split(popup_area);

    // Header
    let header_text = vec![
        Line::from(vec![Span::styled(
            "Nimbus CTL - Help",
            Style::default().fg(Color::Cyan),
        )]),
        Line::from(""),
    ];

    let header = Paragraph::new(header_text).alignment(Alignment::Center);

    f.render_widget(header, inner_area[0]);

    // Keybindings
    let keybindings = get_default_keybindings();
    let keybinding_items: Vec<ListItem> = keybindings
        .into_iter()
        .map(|(key, desc)| {
            ListItem::new(Line::from(vec![
                Span::raw("│ "),
                Span::styled(format!("{:12}", key), Style::default().fg(Color::Green)),
                Span::styled(desc, Style::default().fg(Color::White)),
            ]))
        })
        .collect();

    let keybinding_list =
        List::new(keybinding_items).block(Block::default().borders(Borders::NONE));

    f.render_widget(keybinding_list, inner_area[1]);

    // Footer
    let footer_text = vec![
        Line::from(""),
        Line::from(vec![
            Span::styled("Press ", Style::default().fg(Color::Gray)),
            Span::styled("?", Style::default().fg(Color::Green)),
            Span::styled(" or ", Style::default().fg(Color::Gray)),
            Span::styled("Esc", Style::default().fg(Color::Green)),
            Span::styled(" to close", Style::default().fg(Color::Gray)),
        ]),
    ];

    let footer = Paragraph::new(footer_text).alignment(Alignment::Center);

    f.render_widget(footer, inner_area[2]);
}

fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
