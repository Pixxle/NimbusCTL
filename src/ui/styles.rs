use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders},
};

pub fn get_default_block(title: &str) -> Block {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Gray))
}

pub fn get_selected_block(title: &str) -> Block {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Yellow))
}

pub fn get_error_block(title: &str) -> Block {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Red))
}

pub fn get_success_block(title: &str) -> Block {
    Block::default()
        .title(title)
        .borders(Borders::ALL)
        .border_style(Style::default().fg(Color::Green))
}

pub struct Theme {
    pub background: Color,
    pub foreground: Color,
    pub accent: Color,
    pub error: Color,
    pub warning: Color,
    pub success: Color,
    pub info: Color,
    pub border: Color,
    pub selected_border: Color,
    pub highlight_bg: Color,
}

impl Default for Theme {
    fn default() -> Self {
        Self {
            background: Color::Black,
            foreground: Color::White,
            accent: Color::Cyan,
            error: Color::Red,
            warning: Color::Yellow,
            success: Color::Green,
            info: Color::Blue,
            border: Color::Gray,
            selected_border: Color::Yellow,
            highlight_bg: Color::DarkGray,
        }
    }
}

pub fn get_theme() -> Theme {
    Theme::default()
}

pub fn get_service_color(service: &str) -> Color {
    match service {
        "EC2" => Color::Blue,
        "S3" => Color::Green,
        "RDS" => Color::Yellow,
        "IAM" => Color::Magenta,
        "Secrets" => Color::Red,
        "EKS" => Color::Cyan,
        _ => Color::White,
    }
}

pub fn get_state_color(state: &str) -> Color {
    match state.to_lowercase().as_str() {
        "running" | "active" | "available" | "ok" => Color::Green,
        "stopped" | "inactive" | "unavailable" | "error" => Color::Red,
        "starting" | "stopping" | "pending" | "warning" => Color::Yellow,
        "terminated" | "deleted" => Color::DarkGray,
        _ => Color::Gray,
    }
}