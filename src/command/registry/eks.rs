use crate::aws::types::ServiceType;
use crate::command::commands::{
    Command, CommandAction, CommandCategory, ContextRequirement, ServiceCommand,
};
use crate::command::context::CommandContext;

/// Create EKS-specific commands
pub fn create_eks_commands() -> Vec<Command> {
    let service_type = ServiceType::EKS;
    let mut commands = Vec::new();

    // List commands (no resource selection required)
    commands.push(
        Command::new(
            "service.eks.listclusters".to_string(),
            "List EKS Clusters".to_string(),
            "List all EKS clusters in the current region".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::ListClusters),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "eks".to_string(),
            "list".to_string(),
            "clusters".to_string(),
            "kubernetes".to_string(),
            "k8s".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)]),
    );

    // Create commands (no resource selection required)
    commands.push(
        Command::new(
            "service.eks.createcluster".to_string(),
            "Create EKS Cluster".to_string(),
            "Create a new EKS cluster".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::CreateCluster),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "eks".to_string(),
            "create".to_string(),
            "new".to_string(),
            "cluster".to_string(),
            "kubernetes".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)]),
    );

    // Resource-specific commands (require resource selection)
    let resource_commands = vec![
        (
            ServiceCommand::DeleteCluster,
            vec![
                "delete".to_string(),
                "remove".to_string(),
                "destroy".to_string(),
            ],
        ),
        (
            ServiceCommand::DescribeCluster,
            vec![
                "describe".to_string(),
                "details".to_string(),
                "info".to_string(),
            ],
        ),
        (
            ServiceCommand::UpdateKubeconfig,
            vec![
                "kubeconfig".to_string(),
                "kubectl".to_string(),
                "config".to_string(),
            ],
        ),
        (
            ServiceCommand::ListNodeGroups,
            vec![
                "nodes".to_string(),
                "nodegroups".to_string(),
                "workers".to_string(),
            ],
        ),
    ];

    for (service_command, extra_keywords) in resource_commands {
        let mut keywords = vec![
            "eks".to_string(),
            "cluster".to_string(),
            "kubernetes".to_string(),
        ];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.eks.{:?}", service_command).to_lowercase(),
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

/// Create EKS-specific commands with context awareness
pub fn create_eks_commands_with_context(context: &CommandContext) -> Vec<Command> {
    let service_type = ServiceType::EKS;
    let mut commands = Vec::new();
    let is_service_selected = context.selected_service == Some(service_type);
    let has_resource_selected = context.selected_resource.is_some() && is_service_selected;

    // List commands (no resource selection required)
    commands.push(
        Command::new(
            "service.eks.listclusters".to_string(),
            "List EKS Clusters".to_string(),
            "List all EKS clusters in the current region".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::ListClusters),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "eks".to_string(),
            "list".to_string(),
            "clusters".to_string(),
            "kubernetes".to_string(),
            "k8s".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)])
        .with_enabled(is_service_selected),
    );

    // Create commands (no resource selection required)
    commands.push(
        Command::new(
            "service.eks.createcluster".to_string(),
            "Create EKS Cluster".to_string(),
            "Create a new EKS cluster".to_string(),
            CommandCategory::Service(service_type),
            CommandAction::ExecuteServiceCommand(service_type, ServiceCommand::CreateCluster),
            service_type.icon().to_string(),
        )
        .with_keywords(vec![
            "eks".to_string(),
            "create".to_string(),
            "new".to_string(),
            "cluster".to_string(),
            "kubernetes".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ServiceSelected(service_type)])
        .with_enabled(is_service_selected),
    );

    // Resource-specific commands (require resource selection)
    let resource_commands = vec![
        (
            ServiceCommand::DeleteCluster,
            vec![
                "delete".to_string(),
                "remove".to_string(),
                "destroy".to_string(),
            ],
        ),
        (
            ServiceCommand::DescribeCluster,
            vec![
                "describe".to_string(),
                "details".to_string(),
                "info".to_string(),
            ],
        ),
        (
            ServiceCommand::UpdateKubeconfig,
            vec![
                "kubeconfig".to_string(),
                "kubectl".to_string(),
                "config".to_string(),
            ],
        ),
        (
            ServiceCommand::ListNodeGroups,
            vec![
                "nodes".to_string(),
                "nodegroups".to_string(),
                "workers".to_string(),
            ],
        ),
    ];

    for (service_command, extra_keywords) in resource_commands {
        let mut keywords = vec![
            "eks".to_string(),
            "cluster".to_string(),
            "kubernetes".to_string(),
        ];
        keywords.extend(extra_keywords);

        commands.push(
            Command::new(
                format!("service.eks.{:?}", service_command).to_lowercase(),
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
