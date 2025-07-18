use crate::app::state::AppPage;
use crate::aws::types::ServiceType;
use crate::command::commands::{
    Command, CommandAction, CommandCategory, ContextRequirement, UIElement,
};
use crate::command::context::CommandContext;

mod ec2;
mod eks;
mod general;
mod iam;
mod navigation;
mod profile;
mod rds;
mod region;
mod s3;
mod secrets;

pub use ec2::*;
pub use eks::*;
pub use general::*;
pub use iam::*;
pub use navigation::*;
pub use profile::*;
pub use rds::*;
pub use region::*;
pub use s3::*;
pub use secrets::*;

/// Registry that manages all available commands and provides context-aware filtering
pub struct CommandRegistry {
    commands: Vec<Command>,
}

impl CommandRegistry {
    /// Create a new command registry with all available commands
    pub fn new() -> Self {
        let mut commands = Vec::new();

        // Add navigation commands
        commands.extend(create_navigation_commands());

        // Add profile commands (legacy static commands)
        commands.extend(create_profile_commands());

        // Add region commands (legacy static commands)
        commands.extend(create_region_commands());

        // Add service commands
        commands.extend(Self::create_service_commands());

        // Add general commands
        commands.extend(create_general_commands());

        Self { commands }
    }

    /// Create a new command registry with context-aware commands
    pub fn new_with_context(context: &CommandContext) -> Self {
        let mut commands = Vec::new();

        // Add navigation commands
        commands.extend(create_navigation_commands());

        // Add context-aware profile commands
        commands.extend(create_profile_commands_for_context(context));

        // Add context-aware region commands
        commands.extend(create_region_commands_for_context(context));

        // Add service commands
        commands.extend(Self::create_service_commands());

        // Add general commands
        commands.extend(create_general_commands());

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
        commands.extend(create_navigation_commands());

        // Add context-aware profile commands
        commands.extend(create_profile_commands_for_context(context));

        // Add context-aware region commands
        commands.extend(create_region_commands_for_context(context));

        // Add service commands with context-aware enabling/disabling
        commands.extend(Self::create_service_commands_with_context(context));

        // Add general commands
        commands.extend(create_general_commands());

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

    /// Create service-specific commands with context-aware enabling/disabling
    fn create_service_commands_with_context(context: &CommandContext) -> Vec<Command> {
        let mut commands = Vec::new();

        // Create commands for each service type using dedicated builders
        commands.extend(create_ec2_commands_with_context(context));
        commands.extend(create_s3_commands_with_context(context));
        commands.extend(create_rds_commands_with_context(context));
        commands.extend(create_iam_commands_with_context(context));
        commands.extend(create_secrets_commands_with_context(context));
        commands.extend(create_eks_commands_with_context(context));

        commands
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

    /// Create service-specific commands
    fn create_service_commands() -> Vec<Command> {
        let mut commands = Vec::new();

        // Create commands for each service type using dedicated builders
        commands.extend(create_ec2_commands());
        commands.extend(create_s3_commands());
        commands.extend(create_rds_commands());
        commands.extend(create_iam_commands());
        commands.extend(create_secrets_commands());
        commands.extend(create_eks_commands());

        commands
    }
}

impl Default for CommandRegistry {
    fn default() -> Self {
        Self::new()
    }
}
