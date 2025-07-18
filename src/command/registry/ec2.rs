use crate::aws::types::ServiceType;
use crate::command::commands::{
    Command, CommandAction, CommandCategory, ContextRequirement, ServiceCommand,
};
use crate::command::context::CommandContext;

/// Create EC2-specific commands
pub fn create_ec2_commands() -> Vec<Command> {
    let service_type = ServiceType::EC2;
    let mut commands = Vec::new();

    // List commands (no resource selection required)
    commands.push(
        Command::new(
            "service.ec2.listinstances".to_string(),
            "List EC2 Instances".to_string(),
            "List all EC2 instances in the current region".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::ListInstances),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "ec2".to_string(),
            "list".to_string(),
            "instances".to_string(),
            "show".to_string(),
            "view".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)]),
    );

    // Create commands (no resource selection required)
    commands.push(
        Command::new(
            "service.ec2.createinstance".to_string(),
            "Create EC2 Instance".to_string(),
            "Launch a new EC2 instance".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::CreateInstance),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "ec2".to_string(),
            "create".to_string(),
            "launch".to_string(),
            "new".to_string(),
            "instance".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)]),
    );

    // Resource-specific commands (require resource selection)
    let resource_commands = vec![
        (
            ServiceCommand::StartInstance,
            vec!["start".to_string(), "run".to_string(), "launch".to_string()],
        ),
        (
            ServiceCommand::StopInstance,
            vec![
                "stop".to_string(),
                "halt".to_string(),
                "shutdown".to_string(),
            ],
        ),
        (
            ServiceCommand::RebootInstance,
            vec![
                "reboot".to_string(),
                "restart".to_string(),
                "reset".to_string(),
            ],
        ),
        (
            ServiceCommand::TerminateInstance,
            vec![
                "terminate".to_string(),
                "destroy".to_string(),
                "delete".to_string(),
            ],
        ),
        (
            ServiceCommand::DescribeInstance,
            vec![
                "describe".to_string(),
                "details".to_string(),
                "info".to_string(),
            ],
        ),
    ];

    for (service_command, extra_keywords) in resource_commands {
        let mut keywords = vec!["ec2".to_string(), "instance".to_string()];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.ec2.{:?}", service_command).to_lowercase(),
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

/// Create EC2-specific commands with context awareness
pub fn create_ec2_commands_with_context(context: &CommandContext) -> Vec<Command> {
    let service_type = ServiceType::EC2;
    let mut commands = Vec::new();
    let is_service_selected = context.selected_service == Some(service_type);
    let has_resource_selected = context.selected_resource.is_some() && is_service_selected;

    // List commands (no resource selection required)
    commands.push(
        Command::new(
            "service.ec2.listinstances".to_string(),
            "List EC2 Instances".to_string(),
            "List all EC2 instances in the current region".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::ListInstances),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "ec2".to_string(),
            "list".to_string(),
            "instances".to_string(),
            "show".to_string(),
            "view".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)])
        .with_enabled(is_service_selected),
    );

    // Create commands (no resource selection required)
    commands.push(
        Command::new(
            "service.ec2.createinstance".to_string(),
            "Create EC2 Instance".to_string(),
            "Launch a new EC2 instance".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::CreateInstance),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "ec2".to_string(),
            "create".to_string(),
            "launch".to_string(),
            "new".to_string(),
            "instance".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)])
        .with_enabled(is_service_selected),
    );

    // Resource-specific commands (require resource selection)
    let resource_commands = vec![
        (
            ServiceCommand::StartInstance,
            vec!["start".to_string(), "run".to_string(), "launch".to_string()],
        ),
        (
            ServiceCommand::StopInstance,
            vec![
                "stop".to_string(),
                "halt".to_string(),
                "shutdown".to_string(),
            ],
        ),
        (
            ServiceCommand::RebootInstance,
            vec![
                "reboot".to_string(),
                "restart".to_string(),
                "reset".to_string(),
            ],
        ),
        (
            ServiceCommand::TerminateInstance,
            vec![
                "terminate".to_string(),
                "destroy".to_string(),
                "delete".to_string(),
            ],
        ),
        (
            ServiceCommand::DescribeInstance,
            vec![
                "describe".to_string(),
                "details".to_string(),
                "info".to_string(),
            ],
        ),
    ];

    for (service_command, extra_keywords) in resource_commands {
        let mut keywords = vec!["ec2".to_string(), "instance".to_string()];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.ec2.{:?}", service_command).to_lowercase(),
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
