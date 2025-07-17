use crate::command::commands::Command;
use crate::command::context::CommandContext;

/// State management for the command palette UI
#[derive(Debug, Clone)]
pub struct CommandPalette {
    /// Whether the command palette is currently visible
    pub visible: bool,
    /// Current input text for filtering commands
    pub input: String,
    /// All available commands (unfiltered)
    pub commands: Vec<Command>,
    /// Commands filtered by current input
    pub filtered_commands: Vec<Command>,
    /// Index of currently selected command
    pub selected_index: usize,
    /// Current context for determining available commands
    pub context: CommandContext,
}

impl CommandPalette {
    /// Create a new command palette with the given context
    pub fn new(context: CommandContext) -> Self {
        Self {
            visible: false,
            input: String::new(),
            commands: Vec::new(),
            filtered_commands: Vec::new(),
            selected_index: 0,
            context,
        }
    }

    /// Toggle the visibility of the command palette
    pub fn toggle(&mut self) {
        self.visible = !self.visible;
        if self.visible {
            self.reset_input();
        }
    }

    /// Show the command palette
    pub fn show(&mut self) {
        self.visible = true;
        self.reset_input();
    }

    /// Hide the command palette
    pub fn hide(&mut self) {
        self.visible = false;
        self.reset_input();
    }

    /// Reset input and selection state
    pub fn reset_input(&mut self) {
        self.input.clear();
        self.selected_index = 0;
        self.update_filtered_commands();
    }

    /// Update the input text and refresh filtered commands
    pub fn update_input(&mut self, input: String) {
        self.input = input;
        self.selected_index = 0;
        self.update_filtered_commands();
    }

    /// Add a character to the input
    pub fn add_char(&mut self, c: char) {
        self.input.push(c);
        self.selected_index = 0;
        self.update_filtered_commands();
    }

    /// Remove the last character from input
    pub fn backspace(&mut self) {
        self.input.pop();
        self.selected_index = 0;
        self.update_filtered_commands();
    }

    /// Move selection up
    pub fn select_previous(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    /// Move selection down
    pub fn select_next(&mut self) {
        if self.selected_index < self.filtered_commands.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }

    /// Get the currently selected command
    pub fn get_selected_command(&self) -> Option<&Command> {
        self.filtered_commands.get(self.selected_index)
    }

    /// Update the command context
    pub fn update_context(&mut self, context: CommandContext) {
        self.context = context;
        self.update_filtered_commands();
    }

    /// Set all available commands
    pub fn set_commands(&mut self, commands: Vec<Command>) {
        self.commands = commands;
        self.update_filtered_commands();
    }

    /// Update filtered commands based on current input and context
    fn update_filtered_commands(&mut self) {
        if self.input.is_empty() {
            // Show all applicable commands when no input
            self.filtered_commands = self
                .commands
                .iter()
                .filter(|cmd| self.is_command_applicable(cmd))
                .cloned()
                .collect();
        } else {
            // Filter by fuzzy matching
            let query = self.input.to_lowercase();
            self.filtered_commands = self
                .commands
                .iter()
                .filter(|cmd| self.is_command_applicable(cmd) && self.matches_query(cmd, &query))
                .cloned()
                .collect();
        }

        // Ensure selected index is within bounds
        if self.selected_index >= self.filtered_commands.len() {
            self.selected_index = self.filtered_commands.len().saturating_sub(1);
        }
    }

    /// Check if a command is applicable in the current context
    fn is_command_applicable(&self, command: &Command) -> bool {
        // Check if command is enabled
        if !command.enabled {
            return false;
        }

        // Check context requirements
        self.context
            .satisfies_all_requirements(&command.context_requirements)
    }

    /// Check if a command matches the search query using fuzzy matching
    fn matches_query(&self, command: &Command, query: &str) -> bool {
        // Check name
        if command.name.to_lowercase().contains(query) {
            return true;
        }

        // Check description
        if command.description.to_lowercase().contains(query) {
            return true;
        }

        // Check keywords
        if command
            .keywords
            .iter()
            .any(|keyword| keyword.to_lowercase().contains(query))
        {
            return true;
        }

        // Check category
        if command
            .category
            .display_name()
            .to_lowercase()
            .contains(query)
        {
            return true;
        }

        false
    }

    /// Get the number of filtered commands
    pub fn filtered_count(&self) -> usize {
        self.filtered_commands.len()
    }

    /// Check if there are any filtered commands
    pub fn has_commands(&self) -> bool {
        !self.filtered_commands.is_empty()
    }

    /// Get all filtered commands
    pub fn get_filtered_commands(&self) -> &[Command] {
        &self.filtered_commands
    }

    /// Get the current input text
    pub fn get_input(&self) -> &str {
        &self.input
    }

    /// Get the selected index
    pub fn get_selected_index(&self) -> usize {
        self.selected_index
    }

    /// Check if the palette is visible
    pub fn is_visible(&self) -> bool {
        self.visible
    }
}
