use crate::aws::types::ServiceType;
use crate::app::state::AppState;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone)]
pub struct DashboardWidget {
    pub id: String,
    pub title: String,
    pub widget_type: WidgetType,
    pub enabled: bool,
    pub position: Position,
    pub size: Size,
}

#[derive(Debug, Clone)]
pub struct Position {
    pub x: u16,
    pub y: u16,
}

#[derive(Debug, Clone)]
pub struct Size {
    pub width: u16,
    pub height: u16,
}

#[derive(Debug, Clone)]
pub enum WidgetType {
    FavoriteResources {
        max_items: usize,
        show_recent_first: bool,
    },
    RecentActivity {
        max_items: usize,
        time_window: chrono::Duration,
    },
    ResourceCounts {
        services: Vec<ServiceType>,
        show_percentages: bool,
    },
    QuickActions {
        actions: Vec<QuickAction>,
    },
    RegionOverview {
        show_all_regions: bool,
    },
    TaggedResources {
        tag_filters: Vec<TagFilter>,
    },
}

#[derive(Debug, Clone)]
pub struct QuickAction {
    pub id: String,
    pub name: String,
    pub description: String,
    pub service_type: ServiceType,
    pub action_type: ActionType,
    pub hotkey: Option<char>,
}

#[derive(Debug, Clone)]
pub struct TagFilter {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ActionType {
    Create,
    Read,
    Update,
    Delete,
    Start,
    Stop,
    Restart,
}

pub struct DashboardLayout {
    pub widgets: Vec<DashboardWidget>,
    selected_widget: Option<usize>,
    layout_config: LayoutConfig,
}

#[derive(Debug, Clone)]
pub struct LayoutConfig {
    pub columns: u16,
    pub rows: u16,
    pub padding: u16,
    pub auto_arrange: bool,
}

impl DashboardLayout {
    pub fn new() -> Self {
        Self {
            widgets: Self::default_widgets(),
            selected_widget: None,
            layout_config: LayoutConfig {
                columns: 2,
                rows: 3,
                padding: 1,
                auto_arrange: true,
            },
        }
    }
    
    fn default_widgets() -> Vec<DashboardWidget> {
        vec![
            DashboardWidget {
                id: "favorites".to_string(),
                title: "Favorite Resources".to_string(),
                widget_type: WidgetType::FavoriteResources {
                    max_items: 5,
                    show_recent_first: true,
                },
                enabled: true,
                position: Position { x: 0, y: 0 },
                size: Size { width: 50, height: 40 },
            },
            DashboardWidget {
                id: "recent".to_string(),
                title: "Recent Activity".to_string(),
                widget_type: WidgetType::RecentActivity {
                    max_items: 5,
                    time_window: chrono::Duration::hours(24),
                },
                enabled: true,
                position: Position { x: 50, y: 0 },
                size: Size { width: 50, height: 40 },
            },
            DashboardWidget {
                id: "quick_actions".to_string(),
                title: "Quick Actions".to_string(),
                widget_type: WidgetType::QuickActions {
                    actions: Self::default_quick_actions(),
                },
                enabled: true,
                position: Position { x: 0, y: 40 },
                size: Size { width: 50, height: 30 },
            },
            DashboardWidget {
                id: "region_overview".to_string(),
                title: "Region Overview".to_string(),
                widget_type: WidgetType::RegionOverview {
                    show_all_regions: false,
                },
                enabled: true,
                position: Position { x: 50, y: 40 },
                size: Size { width: 50, height: 30 },
            },
            DashboardWidget {
                id: "service_status".to_string(),
                title: "Service Status".to_string(),
                widget_type: WidgetType::ResourceCounts {
                    services: vec![
                        ServiceType::EC2,
                        ServiceType::S3,
                        ServiceType::RDS,
                        ServiceType::IAM,
                    ],
                    show_percentages: false,
                },
                enabled: true,
                position: Position { x: 0, y: 70 },
                size: Size { width: 100, height: 30 },
            },
        ]
    }
    
    fn default_quick_actions() -> Vec<QuickAction> {
        vec![
            QuickAction {
                id: "launch_ec2".to_string(),
                name: "Launch EC2 Instance".to_string(),
                description: "Launch a new EC2 instance".to_string(),
                service_type: ServiceType::EC2,
                action_type: ActionType::Create,
                hotkey: Some('1'),
            },
            QuickAction {
                id: "create_s3_bucket".to_string(),
                name: "Create S3 Bucket".to_string(),
                description: "Create a new S3 bucket".to_string(),
                service_type: ServiceType::S3,
                action_type: ActionType::Create,
                hotkey: Some('2'),
            },
            QuickAction {
                id: "create_rds_instance".to_string(),
                name: "Create RDS Database".to_string(),
                description: "Create a new RDS database instance".to_string(),
                service_type: ServiceType::RDS,
                action_type: ActionType::Create,
                hotkey: Some('3'),
            },
            QuickAction {
                id: "create_iam_user".to_string(),
                name: "Create IAM User".to_string(),
                description: "Create a new IAM user".to_string(),
                service_type: ServiceType::IAM,
                action_type: ActionType::Create,
                hotkey: Some('4'),
            },
            QuickAction {
                id: "create_secret".to_string(),
                name: "Create Secret".to_string(),
                description: "Create a new secret".to_string(),
                service_type: ServiceType::Secrets,
                action_type: ActionType::Create,
                hotkey: Some('5'),
            },
        ]
    }
    
    pub fn get_widget_by_id(&self, id: &str) -> Option<&DashboardWidget> {
        self.widgets.iter().find(|w| w.id == id)
    }
    
    pub fn get_selected_widget(&self) -> Option<&DashboardWidget> {
        self.selected_widget.and_then(|i| self.widgets.get(i))
    }
    
    pub fn select_widget(&mut self, index: usize) {
        if index < self.widgets.len() {
            self.selected_widget = Some(index);
        }
    }
    
    pub fn select_next_widget(&mut self) {
        let widget_count = self.widgets.len();
        if widget_count > 0 {
            self.selected_widget = Some(match self.selected_widget {
                Some(i) => (i + 1) % widget_count,
                None => 0,
            });
        }
    }
    
    pub fn select_previous_widget(&mut self) {
        let widget_count = self.widgets.len();
        if widget_count > 0 {
            self.selected_widget = Some(match self.selected_widget {
                Some(i) => (i + widget_count - 1) % widget_count,
                None => widget_count - 1,
            });
        }
    }
    
    pub fn get_quick_actions(&self) -> Vec<&QuickAction> {
        self.widgets
            .iter()
            .filter_map(|w| match &w.widget_type {
                WidgetType::QuickActions { actions } => Some(actions.iter()),
                _ => None,
            })
            .flatten()
            .collect()
    }
    
    pub fn execute_quick_action(&self, action_id: &str) -> Option<&QuickAction> {
        self.get_quick_actions()
            .into_iter()
            .find(|a| a.id == action_id)
    }
}