use crate::app::state::AppState;
use crate::command::{Command, CommandCategory};
use ratatui::{
    layout::{Alignment, Constraint, Direction, Layout, Rect},
    style::{Color, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Clear, List, ListItem, Paragraph},
    Frame,
};
use std::collections::HashMap;

/// Draw the command palette overlay
pub fn draw_command_palette(f: &mut Frame, app_state: &AppState) {
    let area = centered_rect(70, 60, f.area());

    // Clear the area
    f.render_widget(Clear, area);

    // Create the layout
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(3), // Search input
            Constraint::Min(0),    // Command list
            Constraint::Length(2), // Help text
        ])
        .split(area);

    // Draw search input
    draw_command_search_input(f, chunks[0], app_state);

    // Draw command list
    draw_command_list(f, chunks[1], app_state);

    // Draw help text
    draw_command_help(f, chunks[2], app_state);
}

/// Draw the command search input field
fn draw_command_search_input(f: &mut Frame, area: Rect, app_state: &AppState) {
    let input_text = if app_state.command_palette.get_input().is_empty() {
        "Type to search commands..."
    } else {
        app_state.command_palette.get_input()
    };

    let input_style = if app_state.command_palette.get_input().is_empty() {
        Style::default().fg(Color::Gray)
    } else {
        Style::default().fg(Color::White)
    };

    let search_text = vec![Line::from(vec![
        Span::styled("⚡ ", Style::default().fg(Color::Yellow)),
        Span::styled(input_text, input_style),
    ])];

    let input_block = Block::default()
        .borders(Borders::ALL)
        .title("Command Palette")
        .title_alignment(Alignment::Center)
        .border_style(Style::default().fg(Color::Magenta));

    let paragraph = Paragraph::new(search_text)
        .block(input_block)
        .alignment(Alignment::Left);

    f.render_widget(paragraph, area);
}

/// Draw the command list with category grouping
fn draw_command_list(f: &mut Frame, area: Rect, app_state: &AppState) {
    let commands = app_state.command_palette.get_filtered_commands();
    let selected_index = app_state.command_palette.get_selected_index();

    if commands.is_empty() {
        // Show "No results" message
        let no_results_text = vec![Line::from(vec![Span::styled(
            "No matching commands found",
            Style::default().fg(Color::Gray),
        )])];

        let no_results_block = Block::default()
            .borders(Borders::ALL)
            .border_style(Style::default().fg(Color::Gray));

        let paragraph = Paragraph::new(no_results_text)
            .block(no_results_block)
            .alignment(Alignment::Center);

        f.render_widget(paragraph, area);
        return;
    }

    // Group commands by category
    let grouped_commands = group_commands_by_category(commands);
    let list_items = create_command_list_items(&grouped_commands, selected_index);

    let commands_block = Block::default()
        .borders(Borders::ALL)
        .title(format!("Commands ({}/{})", commands.len(), commands.len()))
        .title_alignment(Alignment::Left)
        .border_style(Style::default().fg(Color::Gray));

    let list = List::new(list_items).block(commands_block);

    f.render_widget(list, area);
}

/// Group commands by their category for organized display
fn group_commands_by_category(commands: &[Command]) -> Vec<(CommandCategory, Vec<&Command>)> {
    let mut category_map: HashMap<CommandCategory, Vec<&Command>> = HashMap::new();

    // Group commands by category
    for command in commands {
        category_map
            .entry(command.category.clone())
            .or_default()
            .push(command);
    }

    // Convert to sorted vector with preferred category order
    let mut grouped: Vec<(CommandCategory, Vec<&Command>)> = category_map.into_iter().collect();

    // Sort categories by preference
    grouped.sort_by(|a, b| {
        let order_a = category_sort_order(&a.0);
        let order_b = category_sort_order(&b.0);
        order_a.cmp(&order_b)
    });

    grouped
}

/// Define sort order for categories
fn category_sort_order(category: &CommandCategory) -> u8 {
    match category {
        CommandCategory::Navigation => 0,
        CommandCategory::Profile => 1,
        CommandCategory::Region => 2,
        CommandCategory::Service(_) => 3,
        CommandCategory::General => 4,
    }
}

/// Create list items from grouped commands
fn create_command_list_items<'a>(
    grouped_commands: &'a [(CommandCategory, Vec<&'a Command>)],
    selected_index: usize,
) -> Vec<ListItem<'a>> {
    let mut items = Vec::new();
    let mut current_index = 0;

    for (category, commands) in grouped_commands {
        // Add category header if there are multiple categories
        if grouped_commands.len() > 1 {
            let category_header = ListItem::new(vec![Line::from(vec![Span::styled(
                format!("{} {}", category.icon(), category.display_name()),
                Style::default().fg(Color::Cyan).bg(Color::DarkGray),
            )])]);
            items.push(category_header);
        }

        // Add commands in this category
        for command in commands {
            let is_selected = current_index == selected_index;

            let style = if is_selected {
                Style::default().fg(Color::Yellow).bg(Color::DarkGray)
            } else {
                Style::default().fg(Color::White)
            };

            let icon_style = if is_selected {
                Style::default().fg(Color::Yellow)
            } else {
                Style::default().fg(Color::Green)
            };

            let desc_style = if is_selected {
                Style::default().fg(Color::Gray).bg(Color::DarkGray)
            } else {
                Style::default().fg(Color::Gray)
            };

            let enabled_indicator = if command.enabled { "" } else { " (disabled)" };

            let command_item = ListItem::new(vec![
                Line::from(vec![
                    Span::styled(format!("{} ", command.icon), icon_style),
                    Span::styled(format!("{}{}", command.name, enabled_indicator), style),
                ]),
                Line::from(vec![
                    Span::styled("  ", Style::default()), // Indent
                    Span::styled(&command.description, desc_style),
                ]),
            ]);

            items.push(command_item);
            current_index += 1;
        }

        // Add spacing between categories
        if grouped_commands.len() > 1 {
            items.push(ListItem::new(vec![Line::from("")]));
        }
    }

    items
}

/// Draw help text and keyboard shortcuts
fn draw_command_help(f: &mut Frame, area: Rect, app_state: &AppState) {
    let command_count = app_state.command_palette.get_filtered_commands().len();

    let help_text = if command_count > 0 {
        vec![Line::from(vec![
            Span::styled("↑↓ ", Style::default().fg(Color::Green)),
            Span::styled("Navigate  ", Style::default().fg(Color::Gray)),
            Span::styled("Enter ", Style::default().fg(Color::Green)),
            Span::styled("Execute  ", Style::default().fg(Color::Gray)),
            Span::styled("Esc ", Style::default().fg(Color::Green)),
            Span::styled("Cancel  ", Style::default().fg(Color::Gray)),
            Span::styled("Type ", Style::default().fg(Color::Green)),
            Span::styled("Filter", Style::default().fg(Color::Gray)),
        ])]
    } else {
        vec![Line::from(vec![
            Span::styled("Esc ", Style::default().fg(Color::Green)),
            Span::styled("Cancel  ", Style::default().fg(Color::Gray)),
            Span::styled("Type ", Style::default().fg(Color::Green)),
            Span::styled("Search commands", Style::default().fg(Color::Gray)),
        ])]
    };

    let help_block = Block::default()
        .borders(Borders::TOP)
        .border_style(Style::default().fg(Color::DarkGray));

    let help_paragraph = Paragraph::new(help_text)
        .block(help_block)
        .alignment(Alignment::Center);

    f.render_widget(help_paragraph, area);
}

/// Create a centered rectangle for the command palette overlay
fn centered_rect(percent_x: u16, percent_y: u16, r: Rect) -> Rect {
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Percentage((100 - percent_y) / 2),
            Constraint::Percentage(percent_y),
            Constraint::Percentage((100 - percent_y) / 2),
        ])
        .split(r);

    Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage((100 - percent_x) / 2),
            Constraint::Percentage(percent_x),
            Constraint::Percentage((100 - percent_x) / 2),
        ])
        .split(popup_layout[1])[1]
}
