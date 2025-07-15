use crate::aws::types::{AwsProfile, AwsRegion, Resource, ResourceId, ServiceType};
use crate::aws::client::MultiRegionAwsClients;
use crate::aws::profiles::ProfileManager;
use crate::config::user_config::UserConfig;
use crate::dashboard::favorites::FavoritesManager;
use crate::dashboard::widgets::DashboardLayout;
use crate::utils::error::{AppError, Result};
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, serde::Serialize, serde::Deserialize)]
pub enum AppPage {
    Dashboard,
    ResourceList(ServiceType),
    ResourceDetail(ServiceType, ResourceId),
    Settings,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ActivityEntry {
    pub timestamp: chrono::DateTime<chrono::Utc>,
    pub action: String,
    pub resource_id: String,
    pub resource_name: String,
    pub service_type: ServiceType,
    pub region: String,
}

#[derive(Debug, Clone)]
pub struct Notification {
    pub message: String,
    pub level: NotificationLevel,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

#[derive(Debug, Clone)]
pub enum NotificationLevel {
    Info,
    Warning,
    Error,
    Success,
}

#[derive(Debug, Clone)]
pub struct NavigationItem {
    pub name: String,
    pub description: String,
    pub action: NavigationAction,
    pub icon: String,
    pub keywords: Vec<String>,
}

#[derive(Debug, Clone)]
pub enum NavigationAction {
    NavigateToService(ServiceType),
    NavigateToResource(ServiceType, ResourceId),
}

pub struct AppState {
    // Navigation
    pub current_page: AppPage,
    pub page_history: Vec<AppPage>,
    
    // AWS Configuration
    pub current_profile: String,
    pub current_region: String,
    pub available_profiles: Vec<AwsProfile>,
    pub available_regions: Vec<AwsRegion>,
    
    // AWS Clients
    pub aws_clients: Option<MultiRegionAwsClients>,
    pub profile_manager: ProfileManager,
    
    // Dashboard
    pub dashboard_layout: DashboardLayout,
    pub favorites_manager: FavoritesManager,
    pub recent_activity: Vec<ActivityEntry>,
    
    // Resource Data (per region)
    pub resources: HashMap<(String, ServiceType), Vec<Resource>>,
    pub loading_states: HashMap<(String, ServiceType), bool>,
    pub last_refresh: HashMap<(String, ServiceType), SystemTime>,
    
    // UI State
    pub selected_resource: Option<ResourceId>,
    pub help_visible: bool,
    pub settings_visible: bool,
    pub profile_selector_visible: bool,
    pub region_selector_visible: bool,
    pub selected_widget: Option<usize>,
    pub selected_service: Option<ServiceType>,
    pub selected_resource_index: usize,
    
    // Quick Navigation
    pub quick_nav_visible: bool,
    pub quick_nav_input: String,
    pub quick_nav_suggestions: Vec<NavigationItem>,
    pub quick_nav_selected_index: usize,
    
    // User Configuration
    pub user_config: UserConfig,
    
    // Error State
    pub error_message: Option<String>,
    pub notifications: Vec<Notification>,
}

impl AppState {
    pub async fn new() -> Result<Self> {
        let user_config = UserConfig::load().unwrap_or_default();
        let profile_manager = ProfileManager::new()?;
        let available_profiles = profile_manager.get_profiles().into_iter().cloned().collect();
        
        let current_profile = user_config.aws.default_profile.clone();
        let current_region = user_config.aws.default_region.clone();
        
        let available_regions = vec![
            AwsRegion { name: "us-east-1".to_string(), display_name: "US East (N. Virginia)".to_string() },
            AwsRegion { name: "us-west-2".to_string(), display_name: "US West (Oregon)".to_string() },
            AwsRegion { name: "eu-west-1".to_string(), display_name: "Europe (Ireland)".to_string() },
            AwsRegion { name: "ap-southeast-1".to_string(), display_name: "Asia Pacific (Singapore)".to_string() },
        ];

        let favorites_manager = FavoritesManager::new()?;
        let dashboard_layout = DashboardLayout::new();

        // Try to initialize AWS clients
        let aws_clients = match MultiRegionAwsClients::new(&current_profile, &current_region).await {
            Ok(clients) => Some(clients),
            Err(e) => {
                tracing::warn!("Failed to initialize AWS clients: {}", e);
                None
            }
        };

        Ok(Self {
            current_page: user_config.dashboard.default_page.clone(),
            page_history: vec![],
            current_profile,
            current_region,
            available_profiles,
            available_regions,
            aws_clients,
            profile_manager,
            dashboard_layout,
            favorites_manager,
            recent_activity: vec![],
            resources: HashMap::new(),
            loading_states: HashMap::new(),
            last_refresh: HashMap::new(),
            selected_resource: None,
            help_visible: false,
            settings_visible: false,
            profile_selector_visible: false,
            region_selector_visible: false,
            selected_widget: None,
            selected_service: None,
            selected_resource_index: 0,
            quick_nav_visible: false,
            quick_nav_input: String::new(),
            quick_nav_suggestions: vec![],
            quick_nav_selected_index: 0,
            user_config,
            error_message: None,
            notifications: vec![],
        })
    }

    pub async fn handle_input(&mut self, key: KeyEvent) -> Result<()> {
        // Handle quick navigation input first
        if self.quick_nav_visible {
            return self.handle_quick_nav_input(key).await;
        }
        
        match key.code {
            KeyCode::Char('q') => {
                // Handled in main.rs
                Ok(())
            }
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.toggle_quick_nav();
                Ok(())
            }
            KeyCode::Char('h') | KeyCode::Char('H') => {
                self.navigate_to_dashboard();
                Ok(())
            }
            KeyCode::Char('?') => {
                self.help_visible = !self.help_visible;
                Ok(())
            }
            KeyCode::F(2) => {
                self.settings_visible = !self.settings_visible;
                Ok(())
            }
            KeyCode::Char('r') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.region_selector_visible = !self.region_selector_visible;
                Ok(())
            }
            KeyCode::Esc => {
                self.handle_escape();
                Ok(())
            }
            KeyCode::Enter => {
                self.handle_enter().await
            }
            KeyCode::Tab => {
                self.handle_tab();
                Ok(())
            }
            KeyCode::Up => {
                self.handle_up();
                Ok(())
            }
            KeyCode::Down => {
                self.handle_down();
                Ok(())
            }
            KeyCode::Left => {
                self.handle_left();
                Ok(())
            }
            KeyCode::Right => {
                self.handle_right();
                Ok(())
            }
            KeyCode::Char(c) if c.is_ascii_digit() => {
                self.handle_number_key(c);
                Ok(())
            }
            _ => Ok(()),
        }
    }

    pub async fn update(&mut self) -> Result<()> {
        // Update dashboard widgets if needed
        // This would typically refresh data periodically
        Ok(())
    }


    fn navigate_to_dashboard(&mut self) {
        self.page_history.push(self.current_page.clone());
        self.current_page = AppPage::Dashboard;
        self.selected_widget = None;
    }

    fn handle_escape(&mut self) {
        if self.quick_nav_visible {
            self.quick_nav_visible = false;
            self.quick_nav_input.clear();
            self.quick_nav_suggestions.clear();
            self.quick_nav_selected_index = 0;
        } else if self.help_visible {
            self.help_visible = false;
        } else if self.settings_visible {
            self.settings_visible = false;
        } else if self.profile_selector_visible {
            self.profile_selector_visible = false;
        } else if self.region_selector_visible {
            self.region_selector_visible = false;
        } else if let Some(prev_page) = self.page_history.pop() {
            self.current_page = prev_page;
        }
    }

    async fn handle_enter(&mut self) -> Result<()> {
        match &self.current_page {
            AppPage::ResourceList(service_type) => {
                // Navigate to resource detail
                let resource_id = format!("resource-{}", self.selected_resource_index);
                self.page_history.push(self.current_page.clone());
                self.current_page = AppPage::ResourceDetail(*service_type, resource_id);
            }
            _ => {}
        }
        Ok(())
    }

    fn handle_tab(&mut self) {
        match &self.current_page {
            AppPage::Dashboard => {
                let widget_count = self.dashboard_layout.widgets.len();
                if widget_count > 0 {
                    self.selected_widget = Some(match self.selected_widget {
                        Some(i) => (i + 1) % widget_count,
                        None => 0,
                    });
                }
            }
            _ => {}
        }
    }

    fn handle_up(&mut self) {
        match &self.current_page {
            AppPage::ResourceList(_) => {
                if self.selected_resource_index > 0 {
                    self.selected_resource_index -= 1;
                }
            }
            _ => {}
        }
    }

    fn handle_down(&mut self) {
        match &self.current_page {
            AppPage::ResourceList(_) => {
                // This would be bounded by actual resource count
                self.selected_resource_index += 1;
            }
            _ => {}
        }
    }

    fn handle_left(&mut self) {
        // Handle left navigation based on current page
    }

    fn handle_right(&mut self) {
        // Handle right navigation based on current page
    }

    fn handle_number_key(&mut self, digit: char) {
        if let Some(num) = digit.to_digit(10) {
            let action_index = (num as usize).saturating_sub(1);
            
            match &self.current_page {
                AppPage::Dashboard => {
                    // Execute quick action
                    if action_index < self.dashboard_layout.get_quick_actions().len() {
                        self.execute_quick_action(action_index);
                    }
                }
                _ => {}
            }
        }
    }

    fn execute_quick_action(&mut self, _action_index: usize) {
        // This would execute the quick action
        // For now, just add a notification
        self.notifications.push(Notification {
            message: "Quick action executed".to_string(),
            level: NotificationLevel::Info,
            timestamp: chrono::Utc::now(),
        });
    }

    pub fn add_notification(&mut self, message: String, level: NotificationLevel) {
        self.notifications.push(Notification {
            message,
            level,
            timestamp: chrono::Utc::now(),
        });
    }

    pub fn clear_notifications(&mut self) {
        self.notifications.clear();
    }

    pub async fn switch_profile(&mut self, profile_name: &str) -> Result<()> {
        if let Some(profile) = self.available_profiles.iter().find(|p| p.name == profile_name) {
            self.current_profile = profile.name.clone();
            
            // Reinitialize AWS clients with new profile
            match MultiRegionAwsClients::new(&self.current_profile, &self.current_region).await {
                Ok(clients) => {
                    self.aws_clients = Some(clients);
                    self.add_notification(
                        format!("Switched to profile: {}", profile_name),
                        NotificationLevel::Success,
                    );
                }
                Err(e) => {
                    self.add_notification(
                        format!("Failed to switch profile: {}", e),
                        NotificationLevel::Error,
                    );
                }
            }
        }
        Ok(())
    }

    pub async fn switch_region(&mut self, region_name: &str) -> Result<()> {
        if self.available_regions.iter().any(|r| r.name == region_name) {
            self.current_region = region_name.to_string();
            
            // Update AWS clients for new region
            if let Some(clients) = &mut self.aws_clients {
                if let Err(e) = clients.switch_region(region_name).await {
                    self.add_notification(
                        format!("Failed to switch region: {}", e),
                        NotificationLevel::Error,
                    );
                    return Err(e);
                }
            }
            
            self.add_notification(
                format!("Switched to region: {}", region_name),
                NotificationLevel::Success,
            );
        }
        Ok(())
    }

    // Quick Navigation Methods
    fn toggle_quick_nav(&mut self) {
        self.quick_nav_visible = !self.quick_nav_visible;
        if self.quick_nav_visible {
            self.quick_nav_input.clear();
            self.quick_nav_suggestions = self.create_navigation_items();
            self.quick_nav_selected_index = 0;
        }
    }

    async fn handle_quick_nav_input(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.quick_nav_visible = false;
                self.quick_nav_input.clear();
                self.quick_nav_suggestions.clear();
                self.quick_nav_selected_index = 0;
                Ok(())
            }
            KeyCode::Enter => {
                if let Some(item) = self.quick_nav_suggestions.get(self.quick_nav_selected_index).cloned() {
                    self.execute_navigation_action(&item.action).await?;
                    self.quick_nav_visible = false;
                    self.quick_nav_input.clear();
                    self.quick_nav_suggestions.clear();
                    self.quick_nav_selected_index = 0;
                }
                Ok(())
            }
            KeyCode::Up => {
                if self.quick_nav_selected_index > 0 {
                    self.quick_nav_selected_index -= 1;
                }
                Ok(())
            }
            KeyCode::Down => {
                if self.quick_nav_selected_index < self.quick_nav_suggestions.len().saturating_sub(1) {
                    self.quick_nav_selected_index += 1;
                }
                Ok(())
            }
            KeyCode::Char(c) => {
                self.quick_nav_input.push(c);
                self.update_quick_nav_suggestions();
                self.quick_nav_selected_index = 0;
                Ok(())
            }
            KeyCode::Backspace => {
                self.quick_nav_input.pop();
                self.update_quick_nav_suggestions();
                self.quick_nav_selected_index = 0;
                Ok(())
            }
            _ => Ok(()),
        }
    }

    fn create_navigation_items(&self) -> Vec<NavigationItem> {
        ServiceType::all()
            .into_iter()
            .map(|service| NavigationItem {
                name: service.display_name().to_string(),
                description: format!("Browse {} resources", service.display_name()),
                action: NavigationAction::NavigateToService(service),
                icon: service.icon().to_string(),
                keywords: self.get_service_keywords(service),
            })
            .collect()
    }

    fn get_service_keywords(&self, service: ServiceType) -> Vec<String> {
        match service {
            ServiceType::EC2 => vec!["ec2".to_string(), "compute".to_string(), "instances".to_string(), "virtual".to_string()],
            ServiceType::S3 => vec!["s3".to_string(), "storage".to_string(), "bucket".to_string(), "object".to_string()],
            ServiceType::RDS => vec!["rds".to_string(), "database".to_string(), "mysql".to_string(), "postgres".to_string()],
            ServiceType::IAM => vec!["iam".to_string(), "identity".to_string(), "access".to_string(), "users".to_string(), "roles".to_string()],
            ServiceType::Secrets => vec!["secrets".to_string(), "secret".to_string(), "password".to_string(), "keys".to_string()],
            ServiceType::EKS => vec!["eks".to_string(), "kubernetes".to_string(), "k8s".to_string(), "cluster".to_string()],
        }
    }

    fn update_quick_nav_suggestions(&mut self) {
        if self.quick_nav_input.is_empty() {
            self.quick_nav_suggestions = self.create_navigation_items();
        } else {
            let query = self.quick_nav_input.to_lowercase();
            let all_items = self.create_navigation_items();
            
            self.quick_nav_suggestions = all_items
                .into_iter()
                .filter(|item| {
                    let name_match = item.name.to_lowercase().contains(&query);
                    let desc_match = item.description.to_lowercase().contains(&query);
                    let keyword_match = item.keywords.iter().any(|k| k.to_lowercase().contains(&query));
                    
                    name_match || desc_match || keyword_match
                })
                .collect();
        }
    }

    async fn execute_navigation_action(&mut self, action: &NavigationAction) -> Result<()> {
        match action {
            NavigationAction::NavigateToService(service_type) => {
                self.page_history.push(self.current_page.clone());
                self.current_page = AppPage::ResourceList(*service_type);
                self.selected_resource_index = 0;
                Ok(())
            }
            NavigationAction::NavigateToResource(service_type, resource_id) => {
                self.page_history.push(self.current_page.clone());
                self.current_page = AppPage::ResourceDetail(*service_type, resource_id.clone());
                Ok(())
            }
        }
    }
}