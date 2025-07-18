use crate::aws::types::ServiceType;
use crate::command::commands::{
    Command, CommandAction, CommandCategory, ContextRequirement, ServiceCommand,
};
use crate::command::context::CommandContext;

/// Create S3-specific commands
pub fn create_s3_commands() -> Vec<Command> {
    let service_type = ServiceType::S3;
    let mut commands = Vec::new();

    // List commands (no resource selection required)
    commands.push(
        Command::new(
            "service.s3.listbuckets".to_string(),
            "List S3 Buckets".to_string(),
            "List all S3 buckets in the current account".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::ListBuckets),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "s3".to_string(),
            "list".to_string(),
            "buckets".to_string(),
            "show".to_string(),
            "view".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)]),
    );

    // Create commands (no resource selection required)
    commands.push(
        Command::new(
            "service.s3.createbucket".to_string(),
            "Create S3 Bucket".to_string(),
            "Create a new S3 bucket".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::CreateBucket),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "s3".to_string(),
            "create".to_string(),
            "new".to_string(),
            "bucket".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)]),
    );

    // Resource-specific commands (require resource selection)
    let resource_commands = vec![
        (
            ServiceCommand::DeleteBucket,
            vec![
                "delete".to_string(),
                "remove".to_string(),
                "destroy".to_string(),
            ],
        ),
        (
            ServiceCommand::GetBucketInfo,
            vec![
                "info".to_string(),
                "details".to_string(),
                "describe".to_string(),
            ],
        ),
        (
            ServiceCommand::ListObjects,
            vec![
                "list".to_string(),
                "objects".to_string(),
                "contents".to_string(),
            ],
        ),
        (
            ServiceCommand::UploadObject,
            vec!["upload".to_string(), "put".to_string(), "add".to_string()],
        ),
        (
            ServiceCommand::DownloadObject,
            vec![
                "download".to_string(),
                "get".to_string(),
                "retrieve".to_string(),
            ],
        ),
    ];

    for (service_command, extra_keywords) in resource_commands {
        let mut keywords = vec!["s3".to_string(), "bucket".to_string()];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.s3.{:?}", service_command).to_lowercase(),
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

/// Create S3-specific commands with context awareness
pub fn create_s3_commands_with_context(context: &CommandContext) -> Vec<Command> {
    let service_type = ServiceType::S3;
    let mut commands = Vec::new();
    let is_service_selected = context.selected_service == Some(service_type);
    let has_resource_selected = context.selected_resource.is_some() && is_service_selected;

    // List commands (no resource selection required)
    commands.push(
        Command::new(
            "service.s3.listbuckets".to_string(),
            "List S3 Buckets".to_string(),
            "List all S3 buckets in the current account".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::ListBuckets),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "s3".to_string(),
            "list".to_string(),
            "buckets".to_string(),
            "show".to_string(),
            "view".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)])
        .with_enabled(is_service_selected),
    );

    // Create commands (no resource selection required)
    commands.push(
        Command::new(
            "service.s3.createbucket".to_string(),
            "Create S3 Bucket".to_string(),
            "Create a new S3 bucket".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::CreateBucket),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "s3".to_string(),
            "create".to_string(),
            "new".to_string(),
            "bucket".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)])
        .with_enabled(is_service_selected),
    );

    // Resource-specific commands (require resource selection)
    let resource_commands = vec![
        (
            ServiceCommand::DeleteBucket,
            vec![
                "delete".to_string(),
                "remove".to_string(),
                "destroy".to_string(),
            ],
        ),
        (
            ServiceCommand::GetBucketInfo,
            vec![
                "info".to_string(),
                "details".to_string(),
                "describe".to_string(),
            ],
        ),
        (
            ServiceCommand::ListObjects,
            vec![
                "list".to_string(),
                "objects".to_string(),
                "contents".to_string(),
            ],
        ),
        (
            ServiceCommand::UploadObject,
            vec!["upload".to_string(), "put".to_string(), "add".to_string()],
        ),
        (
            ServiceCommand::DownloadObject,
            vec![
                "download".to_string(),
                "get".to_string(),
                "retrieve".to_string(),
            ],
        ),
    ];

    for (service_command, extra_keywords) in resource_commands {
        let mut keywords = vec!["s3".to_string(), "bucket".to_string()];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.s3.{:?}", service_command).to_lowercase(),
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
