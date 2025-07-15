use crate::aws::types::Resource;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_resource_card(f: &mut Frame, area: Rect, resource: &Resource, selected: bool) {
    let style = if selected {
        Style::default().fg(Color::Yellow).bg(Color::DarkGray)
    } else {
        Style::default().fg(Color::White)
    };

    let border_style = if selected {
        Style::default().fg(Color::Yellow)
    } else {
        Style::default().fg(Color::Gray)
    };

    let resource_content = vec![
        Line::from(vec![
            Span::styled(resource.name.as_str(), style),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("ID: ", Style::default().fg(Color::Gray)),
            Span::styled(resource.id.as_str(), Style::default().fg(Color::White)),
        ]),
        Line::from(vec![
            Span::styled("State: ", Style::default().fg(Color::Gray)),
            Span::styled(resource.state.as_str(), get_state_color(&resource.state)),
        ]),
        Line::from(vec![
            Span::styled("Region: ", Style::default().fg(Color::Gray)),
            Span::styled(resource.region.as_str(), Style::default().fg(Color::White)),
        ]),
    ];

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(border_style)
        .title(resource.name.as_str());

    let paragraph = Paragraph::new(resource_content)
        .block(block);

    f.render_widget(paragraph, area);
}

fn get_state_color(state: &str) -> Style {
    match state.to_lowercase().as_str() {
        "running" | "active" | "available" => Style::default().fg(Color::Green),
        "stopped" | "inactive" | "unavailable" => Style::default().fg(Color::Red),
        "starting" | "stopping" | "pending" => Style::default().fg(Color::Yellow),
        _ => Style::default().fg(Color::Gray),
    }
}