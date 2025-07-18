use crate::command::commands::{
    Command, CommandAction, CommandCategory, ContextRequirement, UIElement,
};
use crate::command::context::CommandContext;

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
pub fn create_profile_commands() -> Vec<Command> {
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
