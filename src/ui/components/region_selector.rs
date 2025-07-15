use crate::app::state::AppState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};

pub fn draw_region_selector(f: &mut Frame, area: Rect, app_state: &AppState) {
    let popup_area = centered_rect(60, 70, area);

    f.render_widget(Clear, popup_area);

    let block = Block::default()
        .title("Select AWS Region")
        .borders(Borders::ALL)
        .style(Style::default().bg(Color::Black));

    f.render_widget(block, popup_area);

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(2),
            Constraint::Min(0),
            Constraint::Length(2),
        ])
        .split(popup_area);

    // Header
    let header_text = vec![Line::from(vec![Span::styled(
        "Available Regions:",
        Style::default().fg(Color::White),
    )])];

    let header = Paragraph::new(header_text);
    f.render_widget(header, inner_area[0]);

    // Region list
    let region_items: Vec<ListItem> = app_state
        .available_regions
        .iter()
        .map(|region| {
            let style = if region.name == app_state.current_region {
                Style::default().fg(Color::Yellow).bg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White)
            };

            ListItem::new(Line::from(vec![
                Span::styled(&region.name, style),
                Span::raw(" - "),
                Span::styled(&region.display_name, Style::default().fg(Color::Gray)),
                if region.name == app_state.current_region {
                    Span::styled(" (current)", Style::default().fg(Color::Green))
                } else {
                    Span::raw("")
                },
            ]))
        })
        .collect();

    let region_list = List::new(region_items)
        .block(Block::default().borders(Borders::NONE))
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_widget(region_list, inner_area[1]);

    // Footer
    let footer_text = vec![Line::from(vec![
        Span::styled("Enter", Style::default().fg(Color::Green)),
        Span::styled(" to select, ", Style::default().fg(Color::Gray)),
        Span::styled("Esc", Style::default().fg(Color::Green)),
        Span::styled(" to cancel", Style::default().fg(Color::Gray)),
    ])];

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
