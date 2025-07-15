use crate::app::state::AppState;
use crate::ui::components::header;
use crate::ui::layout::create_dashboard_layout;
use crate::ui::styles::get_default_block;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem},
    Frame,
};

pub fn draw_dashboard(f: &mut Frame, area: Rect, app_state: &AppState) {
    // Use centralized dashboard layout function that covers header to bottom
    let layout_areas = create_dashboard_layout(area);
    // layout_areas: [header, top_left, top_right, bottom_left, bottom_right]

    // Draw header
    header::draw_header(f, layout_areas[0], app_state, "Nimbus CTL");

    // Draw widgets using layout areas
    draw_favorites_widget(f, layout_areas[1], app_state); // Top left
    draw_recent_activity_widget(f, layout_areas[2], app_state); // Top right
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
