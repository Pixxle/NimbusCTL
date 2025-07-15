use crate::app::state::AppState;
use crate::aws::types::{ResourceId, ServiceType};
use crate::ui::styles::get_default_block;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Paragraph},
    Frame,
};

pub fn draw_resource_detail(
    f: &mut Frame,
    area: Rect,
    app_state: &AppState,
    service_type: ServiceType,
    resource_id: &ResourceId,
) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Header
            Constraint::Min(0),    // Main content
        ])
        .split(area);

    // Draw header
    draw_header(f, chunks[0], app_state, service_type, resource_id);

    // Draw resource detail
    draw_resource_detail_content(f, chunks[1], app_state, service_type, resource_id);
}

fn draw_header(
    f: &mut Frame,
    area: Rect,
    app_state: &AppState,
    service_type: ServiceType,
    resource_id: &ResourceId,
) {
    let header_text = vec![Line::from(vec![
        Span::styled(
            format!("{} Resource Details", service_type.display_name()),
            Style::default().fg(Color::Cyan),
        ),
        Span::raw("            "),
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
        Span::styled("[S] Services", Style::default().fg(Color::Green)),
        Span::raw(" "),
        Span::styled("[?] Help", Style::default().fg(Color::Green)),
    ])];

    let header = Paragraph::new(header_text).block(get_default_block(""));

    f.render_widget(header, area);
}

fn draw_resource_detail_content(
    f: &mut Frame,
    area: Rect,
    app_state: &AppState,
    service_type: ServiceType,
    resource_id: &ResourceId,
) {
    let main_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0), Constraint::Length(8)])
        .split(area);

    // Draw resource information
    draw_resource_info(f, main_chunks[0], app_state, service_type, resource_id);

    // Draw actions
    draw_actions_panel(f, main_chunks[1], app_state, service_type);
}

fn draw_resource_info(
    f: &mut Frame,
    area: Rect,
    app_state: &AppState,
    service_type: ServiceType,
    resource_id: &ResourceId,
) {
    let info_lines = match service_type {
        ServiceType::EC2 => vec![
            Line::from(vec![
                Span::styled("Instance ID: ", Style::default().fg(Color::Gray)),
                Span::styled(resource_id, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("State: ", Style::default().fg(Color::Gray)),
                Span::styled("running", Style::default().fg(Color::Green)),
                Span::raw("                "),
                Span::styled("Launch Time: ", Style::default().fg(Color::Gray)),
                Span::styled("2024-01-15 10:30:00", Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Type: ", Style::default().fg(Color::Gray)),
                Span::styled("t3.medium", Style::default().fg(Color::White)),
                Span::raw("               "),
                Span::styled("Uptime: ", Style::default().fg(Color::Gray)),
                Span::styled("23 days, 14 hours", Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Availability Zone: ", Style::default().fg(Color::Gray)),
                Span::styled("us-east-1a", Style::default().fg(Color::White)),
                Span::raw("         "),
                Span::styled("Platform: ", Style::default().fg(Color::Gray)),
                Span::styled("Linux", Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Public IP: ", Style::default().fg(Color::Gray)),
                Span::styled("54.1.2.3", Style::default().fg(Color::White)),
                Span::raw("          "),
                Span::styled("Private IP: ", Style::default().fg(Color::Gray)),
                Span::styled("10.0.1.5", Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("VPC: ", Style::default().fg(Color::Gray)),
                Span::styled("vpc-12345678", Style::default().fg(Color::White)),
                Span::raw("             "),
                Span::styled("Subnet: ", Style::default().fg(Color::Gray)),
                Span::styled("subnet-abcdef12", Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Security Groups: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    "sg-web-servers, sg-default",
                    Style::default().fg(Color::White),
                ),
            ]),
        ],
        ServiceType::S3 => vec![
            Line::from(vec![
                Span::styled("Bucket Name: ", Style::default().fg(Color::Gray)),
                Span::styled(resource_id, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Region: ", Style::default().fg(Color::Gray)),
                Span::styled(&app_state.current_region, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Creation Date: ", Style::default().fg(Color::Gray)),
                Span::styled("2024-01-01", Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Versioning: ", Style::default().fg(Color::Gray)),
                Span::styled("Enabled", Style::default().fg(Color::Green)),
            ]),
            Line::from(vec![
                Span::styled("Encryption: ", Style::default().fg(Color::Gray)),
                Span::styled("Enabled", Style::default().fg(Color::Green)),
            ]),
        ],
        _ => vec![
            Line::from(vec![
                Span::styled("Resource ID: ", Style::default().fg(Color::Gray)),
                Span::styled(resource_id, Style::default().fg(Color::White)),
            ]),
            Line::from(vec![
                Span::styled("Service: ", Style::default().fg(Color::Gray)),
                Span::styled(
                    service_type.display_name(),
                    Style::default().fg(Color::White),
                ),
            ]),
            Line::from(vec![
                Span::styled("Region: ", Style::default().fg(Color::Gray)),
                Span::styled(&app_state.current_region, Style::default().fg(Color::White)),
            ]),
        ],
    };

    let resource_name = match service_type {
        ServiceType::EC2 => "web-server-prod",
        ServiceType::S3 => "assets-prod-bucket",
        _ => "Resource Details",
    };

    let title = format!("Resource: {}", resource_name);
    let paragraph = Paragraph::new(info_lines).block(get_default_block(&title));

    f.render_widget(paragraph, area);
}

fn draw_actions_panel(f: &mut Frame, area: Rect, app_state: &AppState, service_type: ServiceType) {
    let actions = get_service_actions(service_type);

    let action_lines: Vec<Line> = actions
        .into_iter()
        .map(|action| {
            Line::from(vec![
                Span::styled(action.key, Style::default().fg(Color::Green)),
                Span::raw(" "),
                Span::styled(action.description, Style::default().fg(Color::White)),
            ])
        })
        .collect();

    let paragraph = Paragraph::new(action_lines).block(get_default_block("Actions"));

    f.render_widget(paragraph, area);
}

struct ServiceAction {
    key: &'static str,
    description: &'static str,
}

fn get_service_actions(service_type: ServiceType) -> Vec<ServiceAction> {
    match service_type {
        ServiceType::EC2 => vec![
            ServiceAction {
                key: "[S]",
                description: "Stop Instance",
            },
            ServiceAction {
                key: "[R]",
                description: "Reboot Instance",
            },
            ServiceAction {
                key: "[T]",
                description: "Terminate Instance",
            },
            ServiceAction {
                key: "[⭐]",
                description: "Toggle Favorite",
            },
        ],
        ServiceType::S3 => vec![
            ServiceAction {
                key: "[D]",
                description: "Delete Bucket",
            },
            ServiceAction {
                key: "[V]",
                description: "View Objects",
            },
            ServiceAction {
                key: "[⭐]",
                description: "Toggle Favorite",
            },
        ],
        ServiceType::RDS => vec![
            ServiceAction {
                key: "[S]",
                description: "Stop Database",
            },
            ServiceAction {
                key: "[R]",
                description: "Reboot Database",
            },
            ServiceAction {
                key: "[⭐]",
                description: "Toggle Favorite",
            },
        ],
        _ => vec![
            ServiceAction {
                key: "[⭐]",
                description: "Toggle Favorite",
            },
            ServiceAction {
                key: "[E]",
                description: "Edit Resource",
            },
        ],
    }
}
