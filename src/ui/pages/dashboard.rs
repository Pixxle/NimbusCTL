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
            Constraint::Length(3),  // Header
            Constraint::Min(0),     // Main content
        ])
        .split(area);

    // Draw header
    draw_header(f, chunks[0], app_state);

    // Draw main dashboard content
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(50),
            Constraint::Percentage(50),
        ])
        .split(chunks[1]);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(main_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage(60),
            Constraint::Percentage(40),
        ])
        .split(main_chunks[1]);

    // Draw widgets
    draw_favorites_widget(f, left_chunks[0], app_state);
    draw_quick_actions_widget(f, left_chunks[1], app_state);
    draw_recent_activity_widget(f, right_chunks[0], app_state);
    draw_region_overview_widget(f, right_chunks[1], app_state);

    // Draw service status at the bottom
    let bottom_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),
            Constraint::Length(6),
        ])
        .split(chunks[1]);

    draw_service_status_widget(f, bottom_chunks[1], app_state);
}

fn draw_header(f: &mut Frame, area: Rect, app_state: &AppState) {
    let header_text = vec![
        Line::from(vec![
            Span::styled("AWS Cloud Manager", Style::default().fg(Color::Cyan)),
            Span::raw("    "),
            Span::styled("Profile: ", Style::default().fg(Color::Gray)),
            Span::styled(&app_state.current_profile, Style::default().fg(Color::Yellow)),
            Span::raw("    "),
            Span::styled("Region: ", Style::default().fg(Color::Gray)),
            Span::styled(&app_state.current_region, Style::default().fg(Color::Yellow)),
            Span::raw("    "),
            Span::styled("[S] Services", Style::default().fg(Color::Green)),
            Span::raw(" "),
            Span::styled("[?] Help", Style::default().fg(Color::Green)),
        ]),
    ];

    let header = Paragraph::new(header_text)
        .block(get_default_block(""));

    f.render_widget(header, area);
}

fn draw_favorites_widget(f: &mut Frame, area: Rect, app_state: &AppState) {
    let favorites = app_state.favorites_manager.get_favorites();
    
    let items: Vec<ListItem> = if favorites.is_empty() {
        vec![ListItem::new(Line::from(vec![
            Span::styled("No favorite resources", Style::default().fg(Color::Gray)),
        ]))]
    } else {
        favorites.into_iter().take(5).map(|fav| {
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
        }).collect()
    };

    let list = List::new(items)
        .block(get_default_block("Favorite Resources"))
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_widget(list, area);
}

fn draw_recent_activity_widget(f: &mut Frame, area: Rect, app_state: &AppState) {
    let recent_activities = app_state.recent_activity.iter().take(5);
    
    let items: Vec<ListItem> = if app_state.recent_activity.is_empty() {
        vec![ListItem::new(Line::from(vec![
            Span::styled("No recent activity", Style::default().fg(Color::Gray)),
        ]))]
    } else {
        recent_activities.map(|activity| {
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
        }).collect()
    };

    let list = List::new(items)
        .block(get_default_block("Recent Activity"));

    f.render_widget(list, area);
}

fn draw_quick_actions_widget(f: &mut Frame, area: Rect, app_state: &AppState) {
    let quick_actions = app_state.dashboard_layout.get_quick_actions();
    
    let items: Vec<ListItem> = quick_actions.into_iter().take(5).enumerate().map(|(i, action)| {
        ListItem::new(Line::from(vec![
            Span::styled(
                format!("[{}] ", i + 1),
                Style::default().fg(Color::Green),
            ),
            Span::styled(&action.name, Style::default().fg(Color::White)),
        ]))
    }).collect();

    let list = List::new(items)
        .block(get_default_block("Quick Actions"));

    f.render_widget(list, area);
}

fn draw_region_overview_widget(f: &mut Frame, area: Rect, app_state: &AppState) {
    let region_info = vec![
        Line::from(vec![
            Span::styled(&app_state.current_region, Style::default().fg(Color::Yellow)),
            Span::styled(": Active region", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("Available regions: ", Style::default().fg(Color::Gray)),
            Span::styled(
                app_state.available_regions.len().to_string(),
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(region_info)
        .block(get_default_block("Region Overview"));

    f.render_widget(paragraph, area);
}

fn draw_service_status_widget(f: &mut Frame, area: Rect, app_state: &AppState) {
    let service_lines = vec![
        Line::from(vec![
            Span::styled("EC2: ", Style::default().fg(Color::Blue)),
            Span::styled("Loading...", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("S3: ", Style::default().fg(Color::Blue)),
            Span::styled("Loading...", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("RDS: ", Style::default().fg(Color::Blue)),
            Span::styled("Loading...", Style::default().fg(Color::Gray)),
        ]),
        Line::from(vec![
            Span::styled("IAM: ", Style::default().fg(Color::Blue)),
            Span::styled("Loading...", Style::default().fg(Color::Gray)),
        ]),
    ];

    let paragraph = Paragraph::new(service_lines)
        .block(get_default_block("Service Status"));

    f.render_widget(paragraph, area);
}