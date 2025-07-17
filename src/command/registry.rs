use crate::app::state::AppPage;
use crate::aws::types::ServiceType;
use crate::command::commands::{
    Command, CommandAction, CommandCategory, ContextRequirement, ServiceCommand, UIElement,
};
use crate::command::context::CommandContext;

/// Registry that manages all available commands and provides context-aware filtering
pub struct CommandRegistry {
    commands: Vec<Command>,
}

impl CommandRegistry {
    /// Create a new command registry with all available commands
    pub fn new() -> Self {
        let mut commands = Vec::new();

        // Add navigation commands
        commands.extend(Self::create_navigation_commands());

        // Add profile commands (legacy static commands)
        commands.extend(Self::create_profile_commands());

        // Add region commands (legacy static commands)
        commands.extend(Self::create_region_commands());

        // Add service commands
        commands.extend(Self::create_service_commands());

        // Add general commands
        commands.extend(Self::create_general_commands());

        Self { commands }
    }

    /// Create a new command registry with context-aware commands
    pub fn new_with_context(context: &CommandContext) -> Self {
        let mut commands = Vec::new();

        // Add navigation commands
        commands.extend(Self::create_navigation_commands());

        // Add context-aware profile commands
        commands.extend(Self::create_profile_commands_for_context(context));

        // Add context-aware region commands
        commands.extend(Self::create_region_commands_for_context(context));

        // Add service commands
        commands.extend(Self::create_service_commands());

        // Add general commands
        commands.extend(Self::create_general_commands());

        Self { commands }
    }

    /// Get commands that are applicable for the given context
    pub fn get_commands_for_context(&self, context: &CommandContext) -> Vec<Command> {
        self.commands
            .iter()
            .filter(|cmd| self.is_command_applicable(cmd, context))
            .cloned()
            .collect()
    }

    /// Get context-aware commands directly (preferred method)
    pub fn get_context_aware_commands(context: &CommandContext) -> Vec<Command> {
        let mut commands = Vec::new();

        // Add navigation commands
        commands.extend(Self::create_navigation_commands());

        // Add context-aware profile commands
        commands.extend(Self::create_profile_commands_for_context(context));

        // Add context-aware region commands
        commands.extend(Self::create_region_commands_for_context(context));

        // Add service commands
        commands.extend(Self::create_service_commands());

        // Add general commands
        commands.extend(Self::create_general_commands());

        // Filter commands based on context requirements
        commands
            .into_iter()
            .filter(|cmd| {
                // Check if command is enabled
                if !cmd.enabled {
                    return false;
                }
                // Check context requirements
                context.satisfies_all_requirements(&cmd.context_requirements)
            })
            .collect()
    }

    /// Check if a command is applicable in the given context
    fn is_command_applicable(&self, command: &Command, context: &CommandContext) -> bool {
        // Check if command is enabled
        if !command.enabled {
            return false;
        }

        // Check context requirements
        context.satisfies_all_requirements(&command.context_requirements)
    }

    /// Create navigation commands
    fn create_navigation_commands() -> Vec<Command> {
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
                .with_keywords(Self::get_service_keywords(service_type)),
            );
        }

        commands
    }

    /// Create profile switching commands based on available profiles
    pub fn create_profile_commands_for_context(context: &CommandContext) -> Vec<Command> {
        let mut commands = Vec::new();

        // Add generic profile selector toggle command
        commands.push(
            Command::new(
                "profile.selector".to_string(),
                "Show Profile Selector".to_string(),
                "Open profile selector UI".to_string(),
                CommandCategory::Profile,
                CommandAction::ToggleUI(UIElement::ProfileSelector),
                "👤".to_string(),
            )
            .with_keywords(vec![
                "profile".to_string(),
                "selector".to_string(),
                "choose".to_string(),
                "aws".to_string(),
            ])
            .with_context_requirements(vec![ContextRequirement::ProfilesAvailable]),
        );

        // Add specific profile switching commands for each available profile
        for profile in &context.available_profiles {
            // Skip current profile
            if profile.name == context.current_profile {
                continue;
            }

            commands.push(
                Command::new(
                    format!("profile.switch.{}", profile.name),
                    format!("Switch to Profile: {}", profile.name),
                    format!("Switch to AWS profile '{}'", profile.name),
                    CommandCategory::Profile,
                    CommandAction::SwitchProfile(profile.name.clone()),
                    "👤".to_string(),
                )
                .with_keywords(vec![
                    "profile".to_string(),
                    "switch".to_string(),
                    profile.name.clone(),
                    "aws".to_string(),
                    "account".to_string(),
                ])
                .with_context_requirements(vec![ContextRequirement::ProfilesAvailable]),
            );
        }

        commands
    }

    /// Create profile switching commands (legacy method for backward compatibility)
    fn create_profile_commands() -> Vec<Command> {
        vec![Command::new(
            "profile.switch".to_string(),
            "Switch Profile".to_string(),
            "Switch to a different AWS profile".to_string(),
            CommandCategory::Profile,
            CommandAction::ToggleUI(UIElement::ProfileSelector),
            "👤".to_string(),
        )
        .with_keywords(vec![
            "profile".to_string(),
            "switch".to_string(),
            "account".to_string(),
            "aws".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::ProfilesAvailable])]
    }

    /// Create region switching commands based on available regions
    pub fn create_region_commands_for_context(context: &CommandContext) -> Vec<Command> {
        let mut commands = Vec::new();

        // Add generic region selector toggle command
        commands.push(
            Command::new(
                "region.selector".to_string(),
                "Show Region Selector".to_string(),
                "Open region selector UI".to_string(),
                CommandCategory::Region,
                CommandAction::ToggleUI(UIElement::RegionSelector),
                "🌍".to_string(),
            )
            .with_keywords(vec![
                "region".to_string(),
                "selector".to_string(),
                "choose".to_string(),
                "aws".to_string(),
            ])
            .with_context_requirements(vec![ContextRequirement::RegionsAvailable]),
        );

        // Add specific region switching commands for each available region
        for region in &context.available_regions {
            // Skip current region
            if region.name == context.current_region {
                continue;
            }

            commands.push(
                Command::new(
                    format!("region.switch.{}", region.name),
                    format!("Switch to Region: {}", region.display_name),
                    format!(
                        "Switch to AWS region '{}' ({})",
                        region.display_name, region.name
                    ),
                    CommandCategory::Region,
                    CommandAction::SwitchRegion(region.name.clone()),
                    "🌍".to_string(),
                )
                .with_keywords(vec![
                    "region".to_string(),
                    "switch".to_string(),
                    region.name.clone(),
                    region.display_name.clone(),
                    "aws".to_string(),
                    "location".to_string(),
                ])
                .with_context_requirements(vec![ContextRequirement::RegionsAvailable]),
            );
        }

        commands
    }

    /// Create region switching commands (legacy method for backward compatibility)
    fn create_region_commands() -> Vec<Command> {
        vec![Command::new(
            "region.switch".to_string(),
            "Switch Region".to_string(),
            "Switch to a different AWS region".to_string(),
            CommandCategory::Region,
            CommandAction::ToggleUI(UIElement::RegionSelector),
            "🌍".to_string(),
        )
        .with_keywords(vec![
            "region".to_string(),
            "switch".to_string(),
            "location".to_string(),
            "aws".to_string(),
        ])
        .with_context_requirements(vec![ContextRequirement::RegionsAvailable])]
    }

    /// Create service-specific commands
    fn create_service_commands() -> Vec<Command> {
        let mut commands = Vec::new();

        for service_type in ServiceType::all() {
            let service_commands = ServiceCommand::for_service(service_type);

            for service_command in service_commands {
                let mut requirements = vec![ContextRequirement::ServiceSelected(service_type)];

                // Add resource selection requirement if needed
                if service_command.requires_resource_selection() {
                    requirements.push(ContextRequirement::ResourceOfTypeSelected(service_type));
                }

                commands.push(
                    Command::new(
                        format!("service.{:?}.{:?}", service_type, service_command).to_lowercase(),
                        service_command.display_name().to_string(),
                        service_command.description().to_string(),
                        CommandCategory::Service(service_type),
                        CommandAction::ExecuteServiceCommand(service_type, service_command.clone()),
                        service_type.icon().to_string(),
                    )
                    .with_keywords(Self::get_service_command_keywords(&service_command))
                    .with_context_requirements(requirements),
                );
            }
        }

        commands
    }

    /// Create general application commands
    fn create_general_commands() -> Vec<Command> {
        vec![
            Command::new(
                "general.help".to_string(),
                "Show Help".to_string(),
                "Display help information".to_string(),
                CommandCategory::General,
                CommandAction::ShowHelp,
                "❓".to_string(),
            )
            .with_keywords(vec![
                "help".to_string(),
                "info".to_string(),
                "about".to_string(),
                "support".to_string(),
            ]),
            Command::new(
                "general.settings".to_string(),
                "Open Settings".to_string(),
                "Open application settings".to_string(),
                CommandCategory::General,
                CommandAction::OpenSettings,
                "⚙️".to_string(),
            )
            .with_keywords(vec![
                "settings".to_string(),
                "config".to_string(),
                "preferences".to_string(),
                "options".to_string(),
            ]),
        ]
    }

    /// Get keywords for a service type
    fn get_service_keywords(service_type: ServiceType) -> Vec<String> {
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

    /// Get keywords for a service command
    fn get_service_command_keywords(service_command: &ServiceCommand) -> Vec<String> {
        let mut keywords = vec![service_command.display_name().to_lowercase()];

        match service_command {
            ServiceCommand::StartInstance | ServiceCommand::StartDatabase => {
                keywords.extend(vec![
                    "start".to_string(),
                    "run".to_string(),
                    "launch".to_string(),
                ]);
            }
            ServiceCommand::StopInstance | ServiceCommand::StopDatabase => {
                keywords.extend(vec![
                    "stop".to_string(),
                    "halt".to_string(),
                    "shutdown".to_string(),
                ]);
            }
            ServiceCommand::RebootInstance | ServiceCommand::RebootDatabase => {
                keywords.extend(vec![
                    "reboot".to_string(),
                    "restart".to_string(),
                    "reset".to_string(),
                ]);
            }
            ServiceCommand::TerminateInstance => {
                keywords.extend(vec![
                    "terminate".to_string(),
                    "destroy".to_string(),
                    "delete".to_string(),
                ]);
            }
            ServiceCommand::CreateInstance
            | ServiceCommand::CreateBucket
            | ServiceCommand::CreateUser
            | ServiceCommand::CreateRole
            | ServiceCommand::CreateSecret
            | ServiceCommand::CreateCluster => {
                keywords.extend(vec![
                    "create".to_string(),
                    "new".to_string(),
                    "add".to_string(),
                ]);
            }
            ServiceCommand::DeleteBucket
            | ServiceCommand::DeleteUser
            | ServiceCommand::DeleteRole
            | ServiceCommand::DeleteSecret
            | ServiceCommand::DeleteCluster => {
                keywords.extend(vec![
                    "delete".to_string(),
                    "remove".to_string(),
                    "destroy".to_string(),
                ]);
            }
            ServiceCommand::ListInstances
            | ServiceCommand::ListBuckets
            | ServiceCommand::ListDatabases
            | ServiceCommand::ListUsers
            | ServiceCommand::ListRoles
            | ServiceCommand::ListSecrets
            | ServiceCommand::ListClusters => {
                keywords.extend(vec![
                    "list".to_string(),
                    "show".to_string(),
                    "view".to_string(),
                ]);
            }
            _ => {}
        }

        keywords
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}
