use crate::aws::types::ServiceType;
use crate::command::commands::{
    Command, CommandAction, CommandCategory, ContextRequirement, ServiceCommand,
};
use crate::command::context::CommandContext;

/// Create Secrets Manager-specific commands
pub fn create_secrets_commands() -> Vec<Command> {
    let service_type = ServiceType::Secrets;
    let mut commands = Vec::new();

    // List commands (no resource selection required)
    commands.push(
        Command::new(
            "service.secrets.listsecrets".to_string(),
            "List Secrets".to_string(),
            "List all secrets in Secrets Manager".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::ListSecrets),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "secrets".to_string(),
            "list".to_string(),
            "show".to_string(),
            "passwords".to_string(),
            "keys".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)]),
    );

    // Create commands (no resource selection required)
    commands.push(
        Command::new(
            "service.secrets.createsecret".to_string(),
            "Create Secret".to_string(),
            "Create a new secret in Secrets Manager".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::CreateSecret),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "secrets".to_string(),
            "create".to_string(),
            "new".to_string(),
            "password".to_string(),
            "key".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)]),
    );

    // Resource-specific commands (require resource selection)
    let resource_commands = vec![
        (
            ServiceCommand::UpdateSecret,
            vec![
                "update".to_string(),
                "modify".to_string(),
                "change".to_string(),
            ],
        ),
        (
            ServiceCommand::DeleteSecret,
            vec![
                "delete".to_string(),
                "remove".to_string(),
                "destroy".to_string(),
            ],
        ),
        (
            ServiceCommand::DescribeSecret,
            vec![
                "describe".to_string(),
                "details".to_string(),
                "info".to_string(),
            ],
        ),
        (
            ServiceCommand::GetSecretValue,
            vec![
                "get".to_string(),
                "retrieve".to_string(),
                "value".to_string(),
            ],
        ),
    ];

    for (service_command, extra_keywords) in resource_commands {
        let mut keywords = vec!["secrets".to_string(), "secret".to_string()];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.secrets.{:?}", service_command).to_lowercase(),
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

/// Create Secrets Manager-specific commands with context awareness
pub fn create_secrets_commands_with_context(context: &CommandContext) -> Vec<Command> {
    let service_type = ServiceType::Secrets;
    let mut commands = Vec::new();
    let is_service_selected = context.selected_service == Some(service_type);
    let has_resource_selected = context.selected_resource.is_some() && is_service_selected;

    // List commands (no resource selection required)
    commands.push(
        Command::new(
            "service.secrets.listsecrets".to_string(),
            "List Secrets".to_string(),
            "List all secrets in Secrets Manager".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::ListSecrets),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "secrets".to_string(),
            "list".to_string(),
            "show".to_string(),
            "passwords".to_string(),
            "keys".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)])
        .with_enabled(is_service_selected),
    );

    // Create commands (no resource selection required)
    commands.push(
        Command::new(
            "service.secrets.createsecret".to_string(),
            "Create Secret".to_string(),
            "Create a new secret in Secrets Manager".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::CreateSecret),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "secrets".to_string(),
            "create".to_string(),
            "new".to_string(),
            "password".to_string(),
            "key".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)])
        .with_enabled(is_service_selected),
    );

    // Resource-specific commands (require resource selection)
    let resource_commands = vec![
        (
            ServiceCommand::UpdateSecret,
            vec![
                "update".to_string(),
                "modify".to_string(),
                "change".to_string(),
            ],
        ),
        (
            ServiceCommand::DeleteSecret,
            vec![
                "delete".to_string(),
                "remove".to_string(),
                "destroy".to_string(),
            ],
        ),
        (
            ServiceCommand::DescribeSecret,
            vec![
                "describe".to_string(),
                "details".to_string(),
                "info".to_string(),
            ],
        ),
        (
            ServiceCommand::GetSecretValue,
            vec![
                "get".to_string(),
                "retrieve".to_string(),
                "value".to_string(),
            ],
        ),
    ];

    for (service_command, extra_keywords) in resource_commands {
        let mut keywords = vec!["secrets".to_string(), "secret".to_string()];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.secrets.{:?}", service_command).to_lowercase(),
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
