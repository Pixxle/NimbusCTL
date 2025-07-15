use crate::app::state::{AppPage, AppState};
use crate::ui::pages::{dashboard, resource_list, resource_detail, settings};
use crate::ui::components::{status_bar, help_panel, quick_nav};
use ratatui::{
    layout::{Constraint, Direction, Layout},
    Frame,
};

pub fn draw_ui(f: &mut Frame, app_state: &mut AppState) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Min(0),      // Main content
            Constraint::Length(1),   // Status bar
        ])
        .split(f.area());

    // Draw main content based on current page
    match &app_state.current_page {
        AppPage::Dashboard => {
            dashboard::draw_dashboard(f, chunks[0], app_state);
        }
        AppPage::ResourceList(service_type) => {
            resource_list::draw_resource_list(f, chunks[0], app_state, *service_type);
        }
        AppPage::ResourceDetail(service_type, resource_id) => {
            resource_detail::draw_resource_detail(f, chunks[0], app_state, *service_type, resource_id);
        }
        AppPage::Settings => {
            settings::draw_settings(f, chunks[0], app_state);
        }
    }

    // Draw status bar
    status_bar::draw_status_bar(f, chunks[1], app_state);

    // Draw help panel if visible
    if app_state.help_visible {
        help_panel::draw_help_panel(f, f.area(), app_state);
    }

    // Draw quick navigation overlay if visible
    if app_state.quick_nav_visible {
        quick_nav::draw_quick_nav(f, app_state);
    }
}