use crate::app::state::AppState;
use crate::ui::styles::get_default_block;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::Paragraph,
    Frame,
};

pub fn draw_header(f: &mut Frame, area: Rect, app_state: &AppState, page_title: &str) {
    let header_text = vec![Line::from(vec![
        Span::styled(page_title, Style::default().fg(Color::Cyan)),
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
