use crate::app::state::AppState;
use crate::ui::styles::get_default_block;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
    Frame,
};

pub fn draw_dashboard(f: &mut Frame, area: Rect, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
        ])
        .split(area);

    // Draw header
    draw_header(f, chunks[0], app_state);

    // Draw main dashboard content
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(chunks[1]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(main_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(main_chunks[1]);

    // Draw widgets
    draw_favorites_widget(f, left_chunks[0], app_state);

    draw_recent_activity_widget(f, right_chunks[0], app_state);
}

fn draw_header(f: &mut Frame, area: Rect, app_state: &AppState) {
    let header_text = vec![Line::from(vec![
        Span::styled("Nimbus CTL", Style::default().fg(Color::Cyan)),
        Span::raw("    "),
        Span::styled("Profile: ", Style::default().fg(Color::Gray)),
        Span::styled(
            &app_state.current_profile,
            Style::default().fg(Color::Yellow),
        ),
        Span::raw("    "),
        Span::styled("Region: ", Style::default().fg(Color::Gray)),
        Span::styled(
            &app_state.current_region,
            Style::default().fg(Color::Yellow),
        ),
    ])];

    let header = Paragraph::new(header_text).block(get_default_block(""));

    f.render_widget(header, area);
}

fn draw_favorites_widget(f: &mut Frame, area: Rect, app_state: &AppState) {
    let favorites = app_state.favorites_manager.get_favorites();

    let items: Vec<ListItem> = if favorites.is_empty() {
        vec![ListItem::new(Line::from(vec![Span::styled(
            "No favorite resources",
            Style::default().fg(Color::Gray),
        )]))]
    } else {
        favorites
            .into_iter()
            .take(5)
            .map(|fav| {
                ListItem::new(Line::from(vec![
                    Span::styled(
                        format!("[{}] ", fav.service_type.display_name()),
                        Style::default().fg(Color::Blue),
                    ),
                    Span::styled(&fav.name, Style::default().fg(Color::White)),
                    Span::raw(" "),
                    Span::styled(
                        format!("({})", fav.region),
                        Style::default().fg(Color::Gray),
                    ),
                ]))
            })
            .collect()
    };

    let list = List::new(items)
        .block(get_default_block("Favorite Resources"))
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_widget(list, area);
}

fn draw_recent_activity_widget(f: &mut Frame, area: Rect, app_state: &AppState) {
    let recent_activities = app_state.recent_activity.iter().take(5);

    let items: Vec<ListItem> = if app_state.recent_activity.is_empty() {
        vec![ListItem::new(Line::from(vec![Span::styled(
            "No recent activity",
            Style::default().fg(Color::Gray),
        )]))]
    } else {
        recent_activities
            .map(|activity| {
                ListItem::new(Line::from(vec![
                    Span::raw("• "),
                    Span::styled(&activity.action, Style::default().fg(Color::Yellow)),
                    Span::raw(" "),
                    Span::styled(&activity.resource_name, Style::default().fg(Color::White)),
                    Span::raw(" "),
                    Span::styled(
                        format!("({})", activity.region),
                        Style::default().fg(Color::Gray),
                    ),
                ]))
            })
            .collect()
    };

    let list = List::new(items).block(get_default_block("Recent Activity"));

    f.render_widget(list, area);
}
