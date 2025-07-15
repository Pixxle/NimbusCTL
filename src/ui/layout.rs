use ratatui::layout::{Constraint, Direction, Layout, Rect};

pub fn create_main_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),    // Main content
            Constraint::Length(1), // Status bar
        ])
        .split(area)
        .to_vec()
}

pub fn create_header_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Content
        ])
        .split(area)
        .to_vec()
}

pub fn create_dashboard_layout(area: Rect) -> Vec<Rect> {
    // First create header + content vertical split
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
        ])
        .split(area);

    // Then split the main content area horizontally
    let main_chunks = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(50), Constraint::Percentage(50)])
        .split(vertical_chunks[1]);

    vec![
        vertical_chunks[0], // Header area
        main_chunks[0],     // left content
        main_chunks[1],     // right content
    ]
}

pub fn create_resource_list_layout(area: Rect) -> Vec<Rect> {
    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([Constraint::Percentage(60), Constraint::Percentage(40)])
        .split(area)
        .to_vec()
}

pub fn create_settings_layout(area: Rect) -> Vec<Rect> {
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

    vec![
        left_chunks[0],  // Top left
        left_chunks[1],  // Bottom left
        right_chunks[0], // Top right
        right_chunks[1], // Bottom right
    ]
}
