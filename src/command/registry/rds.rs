use crate::aws::types::ServiceType;
use crate::command::commands::{
    Command, CommandAction, CommandCategory, ContextRequirement, ServiceCommand,
};
use crate::command::context::CommandContext;

/// Create RDS-specific commands
pub fn create_rds_commands() -> Vec<Command> {
    let service_type = ServiceType::RDS;
    let mut commands = Vec::new();

    // List commands (no resource selection required)
    commands.push(
        Command::new(
            "service.rds.listdatabases".to_string(),
            "List RDS Databases".to_string(),
            "List all RDS database instances".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::ListDatabases),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "rds".to_string(),
            "list".to_string(),
            "databases".to_string(),
            "db".to_string(),
            "show".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)]),
    );

    // Resource-specific commands (require resource selection)
    let resource_commands = vec![
        (
            ServiceCommand::StartDatabase,
            vec!["start".to_string(), "run".to_string(), "launch".to_string()],
        ),
        (
            ServiceCommand::StopDatabase,
            vec![
                "stop".to_string(),
                "halt".to_string(),
                "shutdown".to_string(),
            ],
        ),
        (
            ServiceCommand::RebootDatabase,
            vec![
                "reboot".to_string(),
                "restart".to_string(),
                "reset".to_string(),
            ],
        ),
        (
            ServiceCommand::DescribeDatabase,
            vec![
                "describe".to_string(),
                "details".to_string(),
                "info".to_string(),
            ],
        ),
        (
            ServiceCommand::CreateSnapshot,
            vec![
                "snapshot".to_string(),
                "backup".to_string(),
                "create".to_string(),
            ],
        ),
        (
            ServiceCommand::RestoreSnapshot,
            vec![
                "restore".to_string(),
                "recover".to_string(),
                "snapshot".to_string(),
            ],
        ),
    ];

    for (service_command, extra_keywords) in resource_commands {
        let mut keywords = vec!["rds".to_string(), "database".to_string(), "db".to_string()];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.rds.{:?}", service_command).to_lowercase(),
                service_command.display_name().to_string(),
                service_command.description().to_string(),
                CommandCategory::Service(service_type),
                CommandAction::ExecuteServiceCommand(service_type, service_command),
                service_type.icon().to_string(),
            )
            .with_keywords(keywords)
            .with_context_requirements(vec![
                ContextRequirement::ServiceSelected(service_type),
                ContextRequirement::ResourceOfTypeSelected(service_type),
            ]),
        );
    }

    commands
}

/// Create RDS-specific commands with context awareness
pub fn create_rds_commands_with_context(context: &CommandContext) -> Vec<Command> {
    let service_type = ServiceType::RDS;
    let mut commands = Vec::new();
    let is_service_selected = context.selected_service == Some(service_type);
    let has_resource_selected = context.selected_resource.is_some() && is_service_selected;

    // List commands (no resource selection required)
    commands.push(
        Command::new(
            "service.rds.listdatabases".to_string(),
            "List RDS Databases".to_string(),
            "List all RDS database instances".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::ListDatabases),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "rds".to_string(),
            "list".to_string(),
            "databases".to_string(),
            "db".to_string(),
            "show".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)])
        .with_enabled(is_service_selected),
    );

    // Resource-specific commands (require resource selection)
    let resource_commands = vec![
        (
            ServiceCommand::StartDatabase,
            vec!["start".to_string(), "run".to_string(), "launch".to_string()],
        ),
        (
            ServiceCommand::StopDatabase,
            vec![
                "stop".to_string(),
                "halt".to_string(),
                "shutdown".to_string(),
            ],
        ),
        (
            ServiceCommand::RebootDatabase,
            vec![
                "reboot".to_string(),
                "restart".to_string(),
                "reset".to_string(),
            ],
        ),
        (
            ServiceCommand::DescribeDatabase,
            vec![
                "describe".to_string(),
                "details".to_string(),
                "info".to_string(),
            ],
        ),
        (
            ServiceCommand::CreateSnapshot,
            vec![
                "snapshot".to_string(),
                "backup".to_string(),
                "create".to_string(),
            ],
        ),
        (
            ServiceCommand::RestoreSnapshot,
            vec![
                "restore".to_string(),
                "recover".to_string(),
                "snapshot".to_string(),
            ],
        ),
    ];

    for (service_command, extra_keywords) in resource_commands {
        let mut keywords = vec!["rds".to_string(), "database".to_string(), "db".to_string()];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.rds.{:?}", service_command).to_lowercase(),
                service_command.display_name().to_string(),
                service_command.description().to_string(),
                CommandCategory::Service(service_type),
                CommandAction::ExecuteServiceCommand(service_type, service_command),
                service_type.icon().to_string(),
            )
            .with_keywords(keywords)
            .with_context_requirements(vec![
                ContextRequirement::ServiceSelected(service_type),
                ContextRequirement::ResourceOfTypeSelected(service_type),
            ])
            .with_enabled(has_resource_selected),
        );
    }

    commands
}
