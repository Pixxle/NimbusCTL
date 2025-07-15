use crate::app::state::AppState;
use crate::aws::types::ServiceType;
use crate::ui::components::header;
use crate::ui::layout::{create_header_layout, create_resource_list_layout};
use crate::ui::styles::get_default_block;
use ratatui::{
    layout::Rect,
    style::{Color, Style},
    text::{Line, Span},
    widgets::{List, ListItem, Paragraph},
    Frame,
};

pub fn draw_resource_list(
    f: &mut Frame,
    area: Rect,
    app_state: &AppState,
    service_type: ServiceType,
) {
    // Use centralized header layout function
    let header_chunks = create_header_layout(area);

    // Draw header
    let page_title = format!("{} Resources", service_type.display_name());
    header::draw_header(f, header_chunks[0], app_state, &page_title);

    // Use centralized resource list layout for main content
    let resource_chunks = create_resource_list_layout(header_chunks[1]);
    // resource_chunks: [list_area, detail_area] (60/40 split)

    draw_resource_list_panel(f, resource_chunks[0], app_state, service_type);
    draw_resource_detail_panel(f, resource_chunks[1], app_state, service_type);
}

fn draw_resource_list_panel(
    f: &mut Frame,
    area: Rect,
    app_state: &AppState,
    service_type: ServiceType,
) {
    let resources = get_mock_resources(service_type);

    let items: Vec<ListItem> = resources
        .into_iter()
        .enumerate()
        .map(|(i, resource)| {
            let style = if i == app_state.selected_resource_index {
                Style::default().bg(Color::DarkGray)
            } else {
                Style::default()
            };

            ListItem::new(Line::from(vec![Span::styled(
                format!("► {}", resource.id),
                style.fg(Color::White),
            )]))
        })
        .collect();

    let title = format!("{} Resources", service_type.display_name());
    let list = List::new(items)
        .block(get_default_block(&title))
        .highlight_style(Style::default().bg(Color::DarkGray));

    f.render_widget(list, area);
}

fn draw_resource_detail_panel(
    f: &mut Frame,
    area: Rect,
    app_state: &AppState,
    service_type: ServiceType,
) {
    let resources = get_mock_resources(service_type);

    let detail_lines = if let Some(resource) = resources.get(app_state.selected_resource_index) {
        vec![
            Line::from(vec![Span::styled(
                "Resource ID:",
                Style::default().fg(Color::Gray),
            )]),
            Line::from(vec![Span::styled(
                &resource.id,
                Style::default().fg(Color::White),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Name:",
                Style::default().fg(Color::Gray),
            )]),
            Line::from(vec![Span::styled(
                &resource.name,
                Style::default().fg(Color::White),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "State:",
                Style::default().fg(Color::Gray),
            )]),
            Line::from(vec![Span::styled(
                &resource.state,
                Style::default().fg(Color::Green),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Region:",
                Style::default().fg(Color::Gray),
            )]),
            Line::from(vec![Span::styled(
                &resource.region,
                Style::default().fg(Color::White),
            )]),
            Line::from(""),
            Line::from(vec![Span::styled(
                "Actions:",
                Style::default().fg(Color::Gray),
            )]),
            Line::from(vec![Span::styled(
                "[Enter] Details",
                Style::default().fg(Color::Green),
            )]),
            Line::from(vec![Span::styled(
                "[F] Favorite",
                Style::default().fg(Color::Green),
            )]),
        ]
    } else {
        vec![Line::from(vec![Span::styled(
            "Select a resource to view details",
            Style::default().fg(Color::Gray),
        )])]
    };

    let paragraph = Paragraph::new(detail_lines).block(get_default_block("Resource Details"));

    f.render_widget(paragraph, area);
}

#[derive(Clone)]
struct MockResource {
    id: String,
    name: String,
    state: String,
    region: String,
}

fn get_mock_resources(service_type: ServiceType) -> Vec<MockResource> {
    match service_type {
        ServiceType::EC2 => vec![
            MockResource {
                id: "i-1234567890abcdef0".to_string(),
                name: "web-server-prod".to_string(),
                state: "running".to_string(),
                region: "us-east-1".to_string(),
            },
            MockResource {
                id: "i-0987654321fedcba9".to_string(),
                name: "api-server-prod".to_string(),
                state: "running".to_string(),
                region: "us-east-1".to_string(),
            },
            MockResource {
                id: "i-abcdef1234567890".to_string(),
                name: "background-worker".to_string(),
                state: "stopped".to_string(),
                region: "us-east-1".to_string(),
            },
        ],
        ServiceType::S3 => vec![
            MockResource {
                id: "assets-prod-bucket".to_string(),
                name: "assets-prod-bucket".to_string(),
                state: "active".to_string(),
                region: "us-east-1".to_string(),
            },
            MockResource {
                id: "logs-bucket".to_string(),
                name: "logs-bucket".to_string(),
                state: "active".to_string(),
                region: "us-east-1".to_string(),
            },
        ],
        ServiceType::RDS => vec![MockResource {
            id: "db-prod-mysql".to_string(),
            name: "production-database".to_string(),
            state: "available".to_string(),
            region: "us-east-1".to_string(),
        }],
        ServiceType::IAM => vec![MockResource {
            id: "user-1".to_string(),
            name: "admin-user".to_string(),
            state: "active".to_string(),
            region: "global".to_string(),
        }],
        ServiceType::Secrets => vec![MockResource {
            id: "secret-1".to_string(),
            name: "db-password".to_string(),
            state: "active".to_string(),
            region: "us-east-1".to_string(),
        }],
        ServiceType::EKS => vec![MockResource {
            id: "cluster-1".to_string(),
            name: "production-cluster".to_string(),
            state: "active".to_string(),
            region: "us-east-1".to_string(),
        }],
    }
}
