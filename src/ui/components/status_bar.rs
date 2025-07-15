use crate::app::state::AppState;
use crate::config::defaults::get_default_keybindings;
use ratatui::{
    layout::{Alignment, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_status_bar(f: &mut Frame, area: Rect, app_state: &AppState) {
    let keybindings = get_default_keybindings();
    
    let status_text = if app_state.user_config.display.show_help_bar {
        keybindings.into_iter().take(8).map(|(key, desc)| {
            vec![
                Span::styled(key, Style::default().fg(Color::Green)),
                Span::raw(" "),
                Span::styled(desc, Style::default().fg(Color::White)),
                Span::raw(" │ "),
            ]
        }).flatten().collect::<Vec<_>>()
    } else {
        vec![
            Span::styled("Press ", Style::default().fg(Color::Gray)),
            Span::styled("?", Style::default().fg(Color::Green)),
            Span::styled(" for help", Style::default().fg(Color::Gray)),
        ]
    };

    let mut status_spans = status_text;
    if let Some(last) = status_spans.last_mut() {
        *last = Span::raw(" ");
    }

    let status_line = Line::from(status_spans);

    let paragraph = Paragraph::new(vec![status_line])
        .style(Style::default().bg(Color::DarkGray))
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}