use crate::app::state::{AppPage, AppState};
use crate::ui::components::{help_panel, quick_nav};
use crate::ui::layout::create_main_layout;
use crate::ui::pages::{dashboard, resource_detail, resource_list, settings};
use ratatui::Frame;

pub fn draw_ui(f: &mut Frame, app_state: &mut AppState) {
    // Use centralized main layout function
    let main_chunks = create_main_layout(f.area());

    // Draw main content based on current page
    match &app_state.current_page {
        AppPage::Dashboard => {
            dashboard::draw_dashboard(f, main_chunks[0], app_state);
        }
        AppPage::ResourceList(service_type) => {
            resource_list::draw_resource_list(f, main_chunks[0], app_state, *service_type);
        }
        AppPage::ResourceDetail(service_type, resource_id) => {
            resource_detail::draw_resource_detail(
                f,
                main_chunks[0],
                app_state,
                *service_type,
                resource_id,
            );
        }
        AppPage::Settings => {
            settings::draw_settings(f, main_chunks[0], app_state);
        }
    }

    // Draw help panel if visible
    if app_state.help_visible {
        help_panel::draw_help_panel(f, f.area(), app_state);
    }

    // Draw quick navigation overlay if visible
    if app_state.quick_nav_visible {
        quick_nav::draw_quick_nav(f, app_state);
    }
}
