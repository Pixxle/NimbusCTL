use crate::command::commands::{Command, CommandAction, CommandCategory};

/// Create general application commands
pub fn create_general_commands() -> Vec<Command> {
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
