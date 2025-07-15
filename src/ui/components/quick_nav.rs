use crate::app::state::AppState;
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear},
    Frame,
};

pub fn draw_quick_nav(f: &mut Frame, app_state: &AppState) {
    let area = centered_rect(60, 50, f.area());
    
    // Clear the area
    f.render_widget(Clear, area);
    
    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3),  // Search input
            Constraint::Min(0),     // Results
        ])
        .split(area);
    
    // Draw search input
    draw_search_input(f, chunks[0], app_state);
    
    // Draw suggestions
    draw_suggestions(f, chunks[1], app_state);
}

fn draw_search_input(f: &mut Frame, area: Rect, app_state: &AppState) {
    let input_text = if app_state.quick_nav_input.is_empty() {
        "Type to search services..."
    } else {
        &app_state.quick_nav_input
    };
    
    let input_style = if app_state.quick_nav_input.is_empty() {
        Style::default().fg(Color::Gray)
    } else {
        Style::default().fg(Color::White)
    };
    
    let search_text = vec![
        Line::from(vec![
            Span::styled("🔍 ", Style::default().fg(Color::Yellow)),
            Span::styled(input_text, input_style),
        ]),
    ];
    
    let input_block = Block::default()
        .borders(Borders::ALL)
        .title("Quick Navigation")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Cyan));
    
    let paragraph = Paragraph::new(search_text)
        .block(input_block)
        .alignment(Alignment::Left);
    
    f.render_widget(paragraph, area);
}

fn draw_suggestions(f: &mut Frame, area: Rect, app_state: &AppState) {
    let suggestions = &app_state.quick_nav_suggestions;
    let selected_index = app_state.quick_nav_selected_index;
    
    if suggestions.is_empty() {
        // Show "No results" message
        let no_results_text = vec![
            Line::from(vec![
                Span::styled("No matching services found", Style::default().fg(Color::Gray)),
            ]),
        ];
        
        let no_results_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Gray));
        
        let paragraph = Paragraph::new(no_results_text)
            .block(no_results_block)
            .alignment(Alignment::Center);
        
        f.render_widget(paragraph, area);
        return;
    }
    
    // Create list items
    let items: Vec<ListItem> = suggestions
        .iter()
        .enumerate()
        .map(|(i, item)| {
            let is_selected = i == selected_index;
            let style = if is_selected {
                Style::default().fg(Color::Yellow).bg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White)
            };
            
            let icon_style = if is_selected {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Cyan)
            };
            
            let desc_style = if is_selected {
                Style::default().fg(Color::Gray).bg(Color::DarkGray)
            } else {
                Style::default().fg(Color::Gray)
            };
            
            ListItem::new(vec![
                Line::from(vec![
                    Span::styled(format!("{} ", item.icon), icon_style),
                    Span::styled(&item.name, style),
                ]),
                Line::from(vec![
                    Span::styled("  ", Style::default()), // Indent
                    Span::styled(&item.description, desc_style),
                ]),
            ])
        })
        .collect();
    
    let suggestions_block = Block::default()
        .borders(Borders::ALL)
        .title(format!("Results ({}/{})", suggestions.len(), suggestions.len()))
        .title_alignment(Alignment::Left)
        .border_style(Style::default().fg(Color::Gray));
    
    let list = List::new(items)
        .block(suggestions_block)
        .highlight_style(Style::default().bg(Color::DarkGray));
    
    f.render_widget(list, area);
    
    // Draw usage hints at the bottom
    if area.height > 5 {
        let hints_area = Rect {
            x: area.x + 1,
            y: area.y + area.height - 2,
            width: area.width - 2,
            height: 1,
        };
        
        let hints_text = vec![
            Line::from(vec![
                Span::styled("↑↓ ", Style::default().fg(Color::Green)),
                Span::styled("Navigate  ", Style::default().fg(Color::Gray)),
                Span::styled("Enter ", Style::default().fg(Color::Green)),
                Span::styled("Select  ", Style::default().fg(Color::Gray)),
                Span::styled("Esc ", Style::default().fg(Color::Green)),
                Span::styled("Cancel", Style::default().fg(Color::Gray)),
            ]),
        ];
        
        let hints_paragraph = Paragraph::new(hints_text)
            .alignment(Alignment::Center);
        
        f.render_widget(hints_paragraph, hints_area);
    }
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