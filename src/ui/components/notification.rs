use crate::app::state::{Notification, NotificationLevel};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, Paragraph},
    Frame,
};

pub fn draw_notification(f: &mut Frame, area: Rect, notification: &Notification) {
    let popup_area = notification_rect(area);

    f.render_widget(Clear, popup_area);

    let (border_color, text_color) = match notification.level {
        NotificationLevel::Info => (Color::Blue, Color::White),
        NotificationLevel::Warning => (Color::Yellow, Color::Black),
        NotificationLevel::Error => (Color::Red, Color::White),
        NotificationLevel::Success => (Color::Green, Color::White),
    };

    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(border_color))
        .style(Style::default().bg(get_notification_bg(&notification.level)));

    let content = vec![Line::from(vec![Span::styled(
        &notification.message,
        Style::default().fg(text_color),
    )])];

    let paragraph = Paragraph::new(content)
        .block(block)
        .alignment(Alignment::Center);

    f.render_widget(paragraph, popup_area);
}

pub fn draw_notifications(f: &mut Frame, area: Rect, notifications: &[Notification]) {
    if notifications.is_empty() {
        return;
    }

    // Show the most recent notification
    if let Some(notification) = notifications.last() {
        draw_notification(f, area, notification);
    }
}

fn notification_rect(area: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(5),
            Constraint::Length(3),
            Constraint::Min(0),
        ])
        .split(area);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20),
            Constraint::Percentage(60),
            Constraint::Percentage(20),
        ])
        .split(popup_layout[1])[1]
}

fn get_notification_bg(level: &NotificationLevel) -> Color {
    match level {
        NotificationLevel::Info => Color::DarkGray,
        NotificationLevel::Warning => Color::Yellow,
        NotificationLevel::Error => Color::Red,
        NotificationLevel::Success => Color::Green,
    }
}
