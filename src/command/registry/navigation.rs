use crate::app::state::AppPage;
use crate::aws::types::ServiceType;
use crate::command::commands::{Command, CommandAction, CommandCategory, ContextRequirement};

/// Create navigation commands
pub fn create_navigation_commands() -> Vec<Command> {
    let mut commands = Vec::new();

    // Dashboard navigation
    commands.push(
        Command::new(
            "nav.dashboard".to_string(),
            "Go to Dashboard".to_string(),
            "Navigate to the main dashboard".to_string(),
            CommandCategory::Navigation,
            CommandAction::NavigateToPage(AppPage::Dashboard),
            "🏠".to_string(),
        )
        .with_keywords(vec![
            "dashboard".to_string(),
            "home".to_string(),
            "main".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::NotOnPage(AppPage::Dashboard)]),
    );

    // Settings navigation
    commands.push(
        Command::new(
            "nav.settings".to_string(),
            "Go to Settings".to_string(),
            "Navigate to application settings".to_string(),
            CommandCategory::Navigation,
            CommandAction::NavigateToPage(AppPage::Settings),
            "⚙️".to_string(),
        )
        .with_keywords(vec![
            "settings".to_string(),
            "config".to_string(),
            "preferences".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::NotOnPage(AppPage::Settings)]),
    );

    // Service navigation commands
    for service_type in ServiceType::all() {
        commands.push(
            Command::new(
                format!("nav.service.{:?}", service_type).to_lowercase(),
                format!("Go to {}", service_type.display_name()),
                format!("Navigate to {} service", service_type.display_name()),
                CommandCategory::Navigation,
                CommandAction::NavigateToService(service_type),
                service_type.icon().to_string(),
            )
            .with_keywords(get_service_keywords(service_type)),
        );
    }

    commands
}

/// Get keywords for a service type
pub fn get_service_keywords(service_type: ServiceType) -> Vec<String> {
    match service_type {
        ServiceType::EC2 => vec![
            "ec2".to_string(),
            "compute".to_string(),
            "instances".to_string(),
            "virtual".to_string(),
            "servers".to_string(),
        ],
        ServiceType::S3 => vec![
            "s3".to_string(),
            "storage".to_string(),
            "bucket".to_string(),
            "object".to_string(),
            "files".to_string(),
        ],
        ServiceType::RDS => vec![
            "rds".to_string(),
            "database".to_string(),
            "mysql".to_string(),
            "postgres".to_string(),
            "db".to_string(),
        ],
        ServiceType::IAM => vec![
            "iam".to_string(),
            "identity".to_string(),
            "access".to_string(),
            "users".to_string(),
            "roles".to_string(),
            "permissions".to_string(),
        ],
        ServiceType::Secrets => vec![
            "secrets".to_string(),
            "secret".to_string(),
            "password".to_string(),
            "keys".to_string(),
            "credentials".to_string(),
        ],
        ServiceType::EKS => vec![
            "eks".to_string(),
            "kubernetes".to_string(),
            "k8s".to_string(),
            "cluster".to_string(),
            "containers".to_string(),
        ],
    }
}
