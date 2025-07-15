use crate::app::state::AppState;
use crate::ui::styles::get_default_block;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_settings(f: &mut Frame, area: Rect, app_state: &AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
        ])
        .split(area);

    // Draw header
    draw_header(f, chunks[0], app_state);

    // Draw settings content
    draw_settings_content(f, chunks[1], app_state);
}

fn draw_header(f: &mut Frame, area: Rect, app_state: &AppState) {
    let header_text = vec![Line::from(vec![
        Span::styled("Settings", Style::default().fg(Color::Cyan)),
        Span::raw("                           "),
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
        Span::raw("    "),
        Span::styled("[?] Help", Style::default().fg(Color::Green)),
    ])];

    let header = Paragraph::new(header_text).block(get_default_block(""));

    f.render_widget(header, area);
}

fn draw_settings_content(f: &mut Frame, area: Rect, app_state: &AppState) {
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(area);

    let left_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[0]);

    let right_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(main_chunks[1]);

    // Draw settings sections
    draw_aws_settings(f, left_chunks[0], app_state);
    draw_display_settings(f, left_chunks[1], app_state);
    draw_dashboard_settings(f, right_chunks[0], app_state);
    draw_behavior_settings(f, right_chunks[1], app_state);
}

fn draw_aws_settings(f: &mut Frame, area: Rect, app_state: &AppState) {
    let aws_lines = vec![
        Line::from(vec![
            Span::styled("Default Profile: ", Style::default().fg(Color::Gray)),
            Span::styled(
                &app_state.user_config.aws.default_profile,
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Default Region: ", Style::default().fg(Color::Gray)),
            Span::styled(
                &app_state.user_config.aws.default_region,
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Auto Refresh: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!("{}s", app_state.user_config.aws.auto_refresh_interval),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Max Requests: ", Style::default().fg(Color::Gray)),
            Span::styled(
                app_state
                    .user_config
                    .aws
                    .max_concurrent_requests
                    .to_string(),
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(aws_lines).block(get_default_block("AWS Settings"));

    f.render_widget(paragraph, area);
}

fn draw_display_settings(f: &mut Frame, area: Rect, app_state: &AppState) {
    let display_lines = vec![
        Line::from(vec![
            Span::styled("Theme: ", Style::default().fg(Color::Gray)),
            Span::styled(
                &app_state.user_config.display.theme,
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Help Bar: ", Style::default().fg(Color::Gray)),
            Span::styled(
                if app_state.user_config.display.show_help_bar {
                    "Yes"
                } else {
                    "No"
                },
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Status Bar: ", Style::default().fg(Color::Gray)),
            Span::styled(
                if app_state.user_config.display.show_status_bar {
                    "Yes"
                } else {
                    "No"
                },
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Unicode: ", Style::default().fg(Color::Gray)),
            Span::styled(
                if app_state.user_config.display.use_unicode_symbols {
                    "Yes"
                } else {
                    "No"
                },
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(display_lines).block(get_default_block("Display Settings"));

    f.render_widget(paragraph, area);
}

fn draw_dashboard_settings(f: &mut Frame, area: Rect, app_state: &AppState) {
    let dashboard_lines = vec![
        Line::from(vec![
            Span::styled("Auto Refresh: ", Style::default().fg(Color::Gray)),
            Span::styled(
                if app_state.user_config.dashboard.auto_refresh_dashboard {
                    "Yes"
                } else {
                    "No"
                },
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Refresh Interval: ", Style::default().fg(Color::Gray)),
            Span::styled(
                format!(
                    "{}s",
                    app_state.user_config.dashboard.dashboard_refresh_interval
                ),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Max Recent: ", Style::default().fg(Color::Gray)),
            Span::styled(
                app_state.user_config.dashboard.max_recent_items.to_string(),
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Max Favorites: ", Style::default().fg(Color::Gray)),
            Span::styled(
                app_state
                    .user_config
                    .dashboard
                    .max_favorite_items
                    .to_string(),
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(dashboard_lines).block(get_default_block("Dashboard Settings"));

    f.render_widget(paragraph, area);
}

fn draw_behavior_settings(f: &mut Frame, area: Rect, app_state: &AppState) {
    let behavior_lines = vec![
        Line::from(vec![
            Span::styled("Auto Refresh: ", Style::default().fg(Color::Gray)),
            Span::styled(
                if app_state.user_config.behavior.auto_refresh_resources {
                    "Yes"
                } else {
                    "No"
                },
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Confirm Actions: ", Style::default().fg(Color::Gray)),
            Span::styled(
                if app_state.user_config.behavior.confirm_destructive_actions {
                    "Yes"
                } else {
                    "No"
                },
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Remember Page: ", Style::default().fg(Color::Gray)),
            Span::styled(
                if app_state.user_config.behavior.remember_last_page {
                    "Yes"
                } else {
                    "No"
                },
                Style::default().fg(Color::White),
            ),
        ]),
        Line::from(vec![
            Span::styled("Save Favorites: ", Style::default().fg(Color::Gray)),
            Span::styled(
                if app_state.user_config.behavior.save_favorites {
                    "Yes"
                } else {
                    "No"
                },
                Style::default().fg(Color::White),
            ),
        ]),
    ];

    let paragraph = Paragraph::new(behavior_lines).block(get_default_block("Behavior Settings"));

    f.render_widget(paragraph, area);
}
