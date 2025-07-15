use crate::app::state::AppState;
use crate::ui::components::header;
use crate::ui::layout::{create_header_layout, create_settings_layout};
use crate::ui::styles::get_default_block;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn draw_settings(f: &mut Frame, area: Rect, app_state: &AppState) {
    // Use centralized header layout function
    let header_chunks = create_header_layout(area);

    // Draw header
    header::draw_header(f, header_chunks[0], app_state, "Settings");

    // Use centralized settings layout for main content
    let settings_areas = create_settings_layout(header_chunks[1]);
    // settings_areas: [top_left, bottom_left, top_right, bottom_right]

    // Draw settings sections using layout areas
    draw_aws_settings(f, settings_areas[0], app_state); // Top left
    draw_display_settings(f, settings_areas[1], app_state); // Bottom left
    draw_dashboard_settings(f, settings_areas[2], app_state); // Top right
    draw_behavior_settings(f, settings_areas[3], app_state); // Bottom right
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
