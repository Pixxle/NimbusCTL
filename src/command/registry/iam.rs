use crate::aws::types::ServiceType;
use crate::command::commands::{
    Command, CommandAction, CommandCategory, ContextRequirement, ServiceCommand,
};
use crate::command::context::CommandContext;

/// Create IAM-specific commands
pub fn create_iam_commands() -> Vec<Command> {
    let service_type = ServiceType::IAM;
    let mut commands = Vec::new();

    // List commands (no resource selection required)
    let list_commands = vec![
        (
            ServiceCommand::ListUsers,
            vec!["users".to_string(), "people".to_string()],
        ),
        (
            ServiceCommand::ListRoles,
            vec!["roles".to_string(), "permissions".to_string()],
        ),
    ];

    for (service_command, extra_keywords) in list_commands {
        let mut keywords = vec!["iam".to_string(), "list".to_string(), "show".to_string()];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.iam.{:?}", service_command).to_lowercase(),
                service_command.display_name().to_string(),
                service_command.description().to_string(),
                CommandCategory::Service(service_type),
                CommandAction::ExecuteServiceCommand(service_type, service_command),
                service_type.icon().to_string(),
            )
            .with_keywords(keywords)
            .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)]),
        );
    }

    // Create commands (no resource selection required)
    let create_commands = vec![
        (
            ServiceCommand::CreateUser,
            vec!["user".to_string(), "person".to_string()],
        ),
        (
            ServiceCommand::CreateRole,
            vec!["role".to_string(), "permission".to_string()],
        ),
    ];

    for (service_command, extra_keywords) in create_commands {
        let mut keywords = vec!["iam".to_string(), "create".to_string(), "new".to_string()];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.iam.{:?}", service_command).to_lowercase(),
                service_command.display_name().to_string(),
                service_command.description().to_string(),
                CommandCategory::Service(service_type),
                CommandAction::ExecuteServiceCommand(service_type, service_command),
                service_type.icon().to_string(),
            )
            .with_keywords(keywords)
            .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)]),
        );
    }

    // Resource-specific commands (require resource selection)
    let resource_commands = vec![
        (
            ServiceCommand::DeleteUser,
            vec![
                "delete".to_string(),
                "remove".to_string(),
                "user".to_string(),
            ],
        ),
        (
            ServiceCommand::DeleteRole,
            vec![
                "delete".to_string(),
                "remove".to_string(),
                "role".to_string(),
            ],
        ),
        (
            ServiceCommand::AttachPolicy,
            vec![
                "attach".to_string(),
                "policy".to_string(),
                "permission".to_string(),
            ],
        ),
        (
            ServiceCommand::DetachPolicy,
            vec![
                "detach".to_string(),
                "policy".to_string(),
                "permission".to_string(),
            ],
        ),
    ];

    for (service_command, extra_keywords) in resource_commands {
        let mut keywords = vec!["iam".to_string()];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.iam.{:?}", service_command).to_lowercase(),
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

/// Create IAM-specific commands with context awareness
pub fn create_iam_commands_with_context(context: &CommandContext) -> Vec<Command> {
    let service_type = ServiceType::IAM;
    let mut commands = Vec::new();
    let is_service_selected = context.selected_service == Some(service_type);
    let has_resource_selected = context.selected_resource.is_some() && is_service_selected;

    // List commands (no resource selection required)
    let list_commands = vec![
        (
            ServiceCommand::ListUsers,
            vec!["users".to_string(), "people".to_string()],
        ),
        (
            ServiceCommand::ListRoles,
            vec!["roles".to_string(), "permissions".to_string()],
        ),
    ];

    for (service_command, extra_keywords) in list_commands {
        let mut keywords = vec!["iam".to_string(), "list".to_string(), "show".to_string()];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.iam.{:?}", service_command).to_lowercase(),
                service_command.display_name().to_string(),
                service_command.description().to_string(),
                CommandCategory::Service(service_type),
                CommandAction::ExecuteServiceCommand(service_type, service_command),
                service_type.icon().to_string(),
            )
            .with_keywords(keywords)
            .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)])
            .with_enabled(is_service_selected),
        );
    }

    // Create commands (no resource selection required)
    let create_commands = vec![
        (
            ServiceCommand::CreateUser,
            vec!["user".to_string(), "person".to_string()],
        ),
        (
            ServiceCommand::CreateRole,
            vec!["role".to_string(), "permission".to_string()],
        ),
    ];

    for (service_command, extra_keywords) in create_commands {
        let mut keywords = vec!["iam".to_string(), "create".to_string(), "new".to_string()];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.iam.{:?}", service_command).to_lowercase(),
                service_command.display_name().to_string(),
                service_command.description().to_string(),
                CommandCategory::Service(service_type),
                CommandAction::ExecuteServiceCommand(service_type, service_command),
                service_type.icon().to_string(),
            )
            .with_keywords(keywords)
            .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)])
            .with_enabled(is_service_selected),
        );
    }

    // Resource-specific commands (require resource selection)
    let resource_commands = vec![
        (
            ServiceCommand::DeleteUser,
            vec![
                "delete".to_string(),
                "remove".to_string(),
                "user".to_string(),
            ],
        ),
        (
            ServiceCommand::DeleteRole,
            vec![
                "delete".to_string(),
                "remove".to_string(),
                "role".to_string(),
            ],
        ),
        (
            ServiceCommand::AttachPolicy,
            vec![
                "attach".to_string(),
                "policy".to_string(),
                "permission".to_string(),
            ],
        ),
        (
            ServiceCommand::DetachPolicy,
            vec![
                "detach".to_string(),
                "policy".to_string(),
                "permission".to_string(),
            ],
        ),
    ];

    for (service_command, extra_keywords) in resource_commands {
        let mut keywords = vec!["iam".to_string()];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.iam.{:?}", service_command).to_lowercase(),
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
