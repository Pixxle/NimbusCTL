use crate::command::commands::{
    Command, CommandAction, CommandCategory, ContextRequirement, UIElement,
};
use crate::command::context::CommandContext;

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
pub fn create_region_commands() -> Vec<Command> {
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
