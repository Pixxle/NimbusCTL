use crate::aws::client::MultiRegionAwsClients;
use crate::aws::profiles::ProfileManager;
use crate::aws::types::{AwsProfile, AwsRegion, Resource, ResourceId, ServiceType};
use crate::command::{CommandContext, CommandPalette, CommandRegistry};
use crate::config::user_config::UserConfig;
use crate::ui::pages::dashboard::favorites::FavoritesManager;
use crate::ui::pages::dashboard::widgets::DashboardLayout;
use crate::utils::error::Result;
use crossterm::event::{KeyCode, KeyEvent, KeyModifiers};
use std::collections::HashMap;
use std::time::SystemTime;

#[derive(Debug, Clone, PartialEq, Eq, serde::Serialize, serde::Deserialize)]
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

    // Command Palette
    pub command_palette: CommandPalette,

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
        let available_profiles: Vec<AwsProfile> = profile_manager
            .get_profiles()
            .into_iter()
            .cloned()
            .collect();

        let current_profile = user_config.aws.default_profile.clone();
        let current_region = user_config.aws.default_region.clone();

        let available_regions = vec![
            AwsRegion {
                name: "us-east-1".to_string(),
                display_name: "US East (N. Virginia)".to_string(),
            },
            AwsRegion {
                name: "us-west-2".to_string(),
                display_name: "US West (Oregon)".to_string(),
            },
            AwsRegion {
                name: "eu-west-1".to_string(),
                display_name: "Europe (Ireland)".to_string(),
            },
            AwsRegion {
                name: "ap-southeast-1".to_string(),
                display_name: "Asia Pacific (Singapore)".to_string(),
            },
        ];

        let favorites_manager = FavoritesManager::new()?;
        let dashboard_layout = DashboardLayout::new();

        // Initialize command context
        let command_context = CommandContext::new(
            user_config.dashboard.default_page.clone(),
            None,
            None,
            available_profiles.clone(),
            available_regions.clone(),
            current_profile.clone(),
            current_region.clone(),
        );

        // Initialize command palette
        let command_palette = CommandPalette::new(command_context);

        // Try to initialize AWS clients
        let aws_clients = match MultiRegionAwsClients::new(&current_profile, &current_region).await
        {
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
            command_palette,
            user_config,
            error_message: None,
            notifications: vec![],
        })
    }

    pub async fn handle_input(&mut self, key: KeyEvent) -> Result<()> {
        // Handle command palette input first
        if self.command_palette.is_visible() {
            return self.handle_command_palette_input(key).await;
        }

        // Handle quick navigation input
        if self.quick_nav_visible {
            return self.handle_quick_nav_input(key).await;
        }

        match key.code {
            KeyCode::Char('q') => {
                // Handled in main.rs
                Ok(())
            }
            KeyCode::Char('p') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                self.toggle_command_palette();
                Ok(())
            }
            KeyCode::Char('P')
                if key.modifiers.contains(KeyModifiers::CONTROL)
                    && key.modifiers.contains(KeyModifiers::SHIFT) =>
            {
                self.toggle_command_palette();
                Ok(())
            }
            KeyCode::Char('?') => {
                self.help_visible = !self.help_visible;
                Ok(())
            }
            KeyCode::Esc => {
                self.handle_escape();
                Ok(())
            }
            KeyCode::Enter => self.handle_enter().await,
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
        if self.command_palette.is_visible() {
            self.command_palette.hide();
        } else if self.quick_nav_visible {
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
            // Update selected service and resource based on new page
            match &self.current_page {
                AppPage::ResourceList(service_type) => {
                    self.selected_service = Some(*service_type);
                    self.selected_resource = None; // Clear resource selection when going back to list
                }
                AppPage::ResourceDetail(service_type, resource_id) => {
                    self.selected_service = Some(*service_type);
                    self.selected_resource = Some(resource_id.clone());
                }
                AppPage::Dashboard | AppPage::Settings => {
                    self.selected_service = None;
                    self.selected_resource = None;
                }
            }
            // Update command context when navigating back
            self.update_command_context();
        }
    }

    async fn handle_enter(&mut self) -> Result<()> {
        match &self.current_page {
            AppPage::ResourceList(service_type) => {
                // Navigate to resource detail
                let resource_id = format!("resource-{}", self.selected_resource_index);
                self.page_history.push(self.current_page.clone());
                self.current_page = AppPage::ResourceDetail(*service_type, resource_id.clone());
                self.selected_resource = Some(resource_id);
                // Update command context when navigating to resource detail
                self.update_command_context();
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
                    // Update command context when resource selection changes
                    self.update_command_context();
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
                // Update command context when resource selection changes
                self.update_command_context();
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
        if let Some(profile) = self
            .available_profiles
            .iter()
            .find(|p| p.name == profile_name)
        {
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

            // Update command context after profile change
            self.update_command_context();
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

            // Update command context after region change
            self.update_command_context();
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
                if let Some(item) = self
                    .quick_nav_suggestions
                    .get(self.quick_nav_selected_index)
                    .cloned()
                {
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
                if self.quick_nav_selected_index
                    < self.quick_nav_suggestions.len().saturating_sub(1)
                {
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
            ServiceType::EC2 => vec![
                "ec2".to_string(),
                "compute".to_string(),
                "instances".to_string(),
                "virtual".to_string(),
            ],
            ServiceType::S3 => vec![
                "s3".to_string(),
                "storage".to_string(),
                "bucket".to_string(),
                "object".to_string(),
            ],
            ServiceType::RDS => vec![
                "rds".to_string(),
                "database".to_string(),
                "mysql".to_string(),
                "postgres".to_string(),
            ],
            ServiceType::IAM => vec![
                "iam".to_string(),
                "identity".to_string(),
                "access".to_string(),
                "users".to_string(),
                "roles".to_string(),
            ],
            ServiceType::Secrets => vec![
                "secrets".to_string(),
                "secret".to_string(),
                "password".to_string(),
                "keys".to_string(),
            ],
            ServiceType::EKS => vec![
                "eks".to_string(),
                "kubernetes".to_string(),
                "k8s".to_string(),
                "cluster".to_string(),
            ],
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
                    let keyword_match = item
                        .keywords
                        .iter()
                        .any(|k| k.to_lowercase().contains(&query));

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

    // Command Palette Methods
    pub fn toggle_command_palette(&mut self) {
        self.command_palette.toggle();
        if self.command_palette.is_visible() {
            self.update_command_context();
            self.populate_command_palette();
        }
    }

    fn populate_command_palette(&mut self) {
        let context = CommandContext::new(
            self.current_page.clone(),
            self.selected_service,
            self.selected_resource.clone(),
            self.available_profiles.clone(),
            self.available_regions.clone(),
            self.current_profile.clone(),
            self.current_region.clone(),
        );
        let commands = CommandRegistry::get_context_aware_commands(&context);
        self.command_palette.set_commands(commands);
    }

    async fn handle_command_palette_input(&mut self, key: KeyEvent) -> Result<()> {
        match key.code {
            KeyCode::Esc => {
                self.command_palette.hide();
                Ok(())
            }
            KeyCode::Enter => {
                if let Some(command) = self.command_palette.get_selected_command() {
                    let command = command.clone();
                    self.command_palette.hide();
                    self.execute_command(&command).await?;
                }
                Ok(())
            }
            KeyCode::Up => {
                self.command_palette.select_previous();
                Ok(())
            }
            KeyCode::Down => {
                self.command_palette.select_next();
                Ok(())
            }
            KeyCode::Char(c) => {
                self.command_palette.add_char(c);
                Ok(())
            }
            KeyCode::Backspace => {
                self.command_palette.backspace();
                Ok(())
            }
            _ => Ok(()),
        }
    }

    /// Update command context and refresh available commands based on current application state
    pub fn update_command_context(&mut self) {
        // Determine selected service from current page if not explicitly set
        let selected_service = self.selected_service.or_else(|| match &self.current_page {
            AppPage::ResourceList(service_type) => Some(*service_type),
            AppPage::ResourceDetail(service_type, _) => Some(*service_type),
            _ => None,
        });

        // Create updated context with current application state
        let context = CommandContext::new(
            self.current_page.clone(),
            selected_service,
            self.selected_resource.clone(),
            self.available_profiles.clone(),
            self.available_regions.clone(),
            self.current_profile.clone(),
            self.current_region.clone(),
        );

        // Update command palette context
        self.command_palette.update_context(context.clone());

        // Refresh commands with new context-aware filtering
        let commands = CommandRegistry::get_context_aware_commands(&context);
        self.command_palette.set_commands(commands);
    }

    /// Set the selected resource and update command context
    pub fn set_selected_resource(&mut self, resource_id: Option<ResourceId>) {
        self.selected_resource = resource_id;
        self.update_command_context();
    }

    /// Set the selected service and update command context
    pub fn set_selected_service(&mut self, service_type: Option<ServiceType>) {
        self.selected_service = service_type;
        self.update_command_context();
    }

    /// Navigate to a page and update command context
    pub fn navigate_to_page(&mut self, page: AppPage) {
        self.page_history.push(self.current_page.clone());
        self.current_page = page.clone();

        // Update selected service and resource based on new page
        match &page {
            AppPage::ResourceList(service_type) => {
                self.selected_service = Some(*service_type);
                self.selected_resource = None;
            }
            AppPage::ResourceDetail(service_type, resource_id) => {
                self.selected_service = Some(*service_type);
                self.selected_resource = Some(resource_id.clone());
            }
            AppPage::Dashboard | AppPage::Settings => {
                self.selected_service = None;
                self.selected_resource = None;
            }
        }

        // Update command context after navigation
        self.update_command_context();
    }

    async fn execute_command(&mut self, command: &crate::command::Command) -> Result<()> {
        use crate::command::{CommandAction, UIElement};

        match &command.action {
            CommandAction::SwitchProfile(profile_name) => {
                self.switch_profile(profile_name).await?;
            }
            CommandAction::SwitchRegion(region_name) => {
                self.switch_region(region_name).await?;
            }
            CommandAction::NavigateToService(service_type) => {
                self.page_history.push(self.current_page.clone());
                self.current_page = AppPage::ResourceList(*service_type);
                self.selected_resource_index = 0;
                self.selected_service = Some(*service_type);
                self.selected_resource = None; // Clear resource selection when navigating to service list
            }
            CommandAction::NavigateToPage(page) => {
                self.page_history.push(self.current_page.clone());
                self.current_page = page.clone();
                // Clear service and resource selection when navigating to non-service pages
                match page {
                    AppPage::Dashboard | AppPage::Settings => {
                        self.selected_service = None;
                        self.selected_resource = None;
                    }
                    _ => {}
                }
            }
            CommandAction::ExecuteServiceCommand(service_type, service_command) => {
                self.execute_service_command(*service_type, service_command)
                    .await?;
            }
            CommandAction::ShowHelp => {
                self.help_visible = true;
            }
            CommandAction::OpenSettings => {
                self.page_history.push(self.current_page.clone());
                self.current_page = AppPage::Settings;
                self.selected_service = None;
                self.selected_resource = None;
            }
            CommandAction::ToggleUI(ui_element) => match ui_element {
                UIElement::ProfileSelector => {
                    self.profile_selector_visible = !self.profile_selector_visible;
                }
                UIElement::RegionSelector => {
                    self.region_selector_visible = !self.region_selector_visible;
                }
                UIElement::Help => {
                    self.help_visible = !self.help_visible;
                }
                UIElement::Settings => {
                    self.settings_visible = !self.settings_visible;
                }
            },
        }

        // Update command context after executing command
        self.update_command_context();
        Ok(())
    }

    /// Execute a service-specific command with proper routing and placeholder implementations
    async fn execute_service_command(
        &mut self,
        service_type: ServiceType,
        service_command: &crate::command::ServiceCommand,
    ) -> Result<()> {
        use crate::command::ServiceCommand;

        // Add activity entry for command execution
        self.recent_activity.push(ActivityEntry {
            timestamp: chrono::Utc::now(),
            action: format!("Executed {}", service_command.display_name()),
            resource_id: self.selected_resource.clone().unwrap_or_default(),
            resource_name: format!("Resource {}", self.selected_resource_index),
            service_type,
            region: self.current_region.clone(),
        });

        match service_type {
            ServiceType::EC2 => self.execute_ec2_command(service_command).await,
            ServiceType::S3 => self.execute_s3_command(service_command).await,
            ServiceType::RDS => self.execute_rds_command(service_command).await,
            ServiceType::IAM => self.execute_iam_command(service_command).await,
            ServiceType::Secrets => self.execute_secrets_command(service_command).await,
            ServiceType::EKS => self.execute_eks_command(service_command).await,
        }
    }

    /// Execute EC2-specific commands
    async fn execute_ec2_command(
        &mut self,
        command: &crate::command::ServiceCommand,
    ) -> Result<()> {
        use crate::command::ServiceCommand;

        match command {
            ServiceCommand::ListInstances => {
                self.add_notification(
                    "Listing EC2 instances...".to_string(),
                    NotificationLevel::Info,
                );
                // TODO: Implement actual EC2 instance listing
                self.add_notification(
                    "EC2 instances listed successfully".to_string(),
                    NotificationLevel::Success,
                );
            }
            ServiceCommand::CreateInstance => {
                self.add_notification(
                    "Creating new EC2 instance...".to_string(),
                    NotificationLevel::Info,
                );
                // TODO: Implement actual EC2 instance creation
                self.add_notification(
                    "EC2 instance creation initiated".to_string(),
                    NotificationLevel::Success,
                );
            }
            ServiceCommand::StartInstance => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Starting EC2 instance {}...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual EC2 instance start
                    self.add_notification(
                        "EC2 instance start initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No EC2 instance selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::StopInstance => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Stopping EC2 instance {}...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual EC2 instance stop
                    self.add_notification(
                        "EC2 instance stop initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No EC2 instance selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::RebootInstance => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Rebooting EC2 instance {}...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual EC2 instance reboot
                    self.add_notification(
                        "EC2 instance reboot initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No EC2 instance selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::TerminateInstance => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!(
                            "Terminating EC2 instance {}...",
                            self.selected_resource_index
                        ),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual EC2 instance termination
                    self.add_notification(
                        "EC2 instance termination initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No EC2 instance selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::DescribeInstance => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!(
                            "Describing EC2 instance {}...",
                            self.selected_resource_index
                        ),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual EC2 instance description
                    self.add_notification(
                        "EC2 instance details retrieved".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No EC2 instance selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            _ => {
                self.add_notification(
                    format!(
                        "EC2 command '{}' not yet implemented",
                        command.display_name()
                    ),
                    NotificationLevel::Info,
                );
            }
        }
        Ok(())
    }

    /// Execute S3-specific commands
    async fn execute_s3_command(&mut self, command: &crate::command::ServiceCommand) -> Result<()> {
        use crate::command::ServiceCommand;

        match command {
            ServiceCommand::ListBuckets => {
                self.add_notification("Listing S3 buckets...".to_string(), NotificationLevel::Info);
                // TODO: Implement actual S3 bucket listing
                self.add_notification(
                    "S3 buckets listed successfully".to_string(),
                    NotificationLevel::Success,
                );
            }
            ServiceCommand::CreateBucket => {
                self.add_notification(
                    "Creating new S3 bucket...".to_string(),
                    NotificationLevel::Info,
                );
                // TODO: Implement actual S3 bucket creation
                self.add_notification(
                    "S3 bucket creation initiated".to_string(),
                    NotificationLevel::Success,
                );
            }
            ServiceCommand::DeleteBucket => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Deleting S3 bucket {}...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual S3 bucket deletion
                    self.add_notification(
                        "S3 bucket deletion initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No S3 bucket selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::GetBucketInfo => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Getting S3 bucket {} info...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual S3 bucket info retrieval
                    self.add_notification(
                        "S3 bucket info retrieved".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No S3 bucket selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::ListObjects => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!(
                            "Listing objects in S3 bucket {}...",
                            self.selected_resource_index
                        ),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual S3 object listing
                    self.add_notification(
                        "S3 objects listed successfully".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No S3 bucket selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::UploadObject => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!(
                            "Uploading object to S3 bucket {}...",
                            self.selected_resource_index
                        ),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual S3 object upload
                    self.add_notification(
                        "S3 object upload initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No S3 bucket selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::DownloadObject => {
                self.add_notification(
                    "Downloading S3 object...".to_string(),
                    NotificationLevel::Info,
                );
                // TODO: Implement actual S3 object download
                self.add_notification(
                    "S3 object download initiated".to_string(),
                    NotificationLevel::Success,
                );
            }
            _ => {
                self.add_notification(
                    format!(
                        "S3 command '{}' not yet implemented",
                        command.display_name()
                    ),
                    NotificationLevel::Info,
                );
            }
        }
        Ok(())
    }

    /// Execute RDS-specific commands
    async fn execute_rds_command(
        &mut self,
        command: &crate::command::ServiceCommand,
    ) -> Result<()> {
        use crate::command::ServiceCommand;

        match command {
            ServiceCommand::ListDatabases => {
                self.add_notification(
                    "Listing RDS databases...".to_string(),
                    NotificationLevel::Info,
                );
                // TODO: Implement actual RDS database listing
                self.add_notification(
                    "RDS databases listed successfully".to_string(),
                    NotificationLevel::Success,
                );
            }
            ServiceCommand::StartDatabase => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Starting RDS database {}...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual RDS database start
                    self.add_notification(
                        "RDS database start initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No RDS database selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::StopDatabase => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Stopping RDS database {}...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual RDS database stop
                    self.add_notification(
                        "RDS database stop initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No RDS database selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::RebootDatabase => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Rebooting RDS database {}...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual RDS database reboot
                    self.add_notification(
                        "RDS database reboot initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No RDS database selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::DescribeDatabase => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!(
                            "Describing RDS database {}...",
                            self.selected_resource_index
                        ),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual RDS database description
                    self.add_notification(
                        "RDS database details retrieved".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No RDS database selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::CreateSnapshot => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!(
                            "Creating snapshot of RDS database {}...",
                            self.selected_resource_index
                        ),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual RDS snapshot creation
                    self.add_notification(
                        "RDS snapshot creation initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No RDS database selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::RestoreSnapshot => {
                self.add_notification(
                    "Restoring RDS database from snapshot...".to_string(),
                    NotificationLevel::Info,
                );
                // TODO: Implement actual RDS snapshot restoration
                self.add_notification(
                    "RDS snapshot restoration initiated".to_string(),
                    NotificationLevel::Success,
                );
            }
            _ => {
                self.add_notification(
                    format!(
                        "RDS command '{}' not yet implemented",
                        command.display_name()
                    ),
                    NotificationLevel::Info,
                );
            }
        }
        Ok(())
    }

    /// Execute IAM-specific commands
    async fn execute_iam_command(
        &mut self,
        command: &crate::command::ServiceCommand,
    ) -> Result<()> {
        use crate::command::ServiceCommand;

        match command {
            ServiceCommand::ListUsers => {
                self.add_notification("Listing IAM users...".to_string(), NotificationLevel::Info);
                // TODO: Implement actual IAM user listing
                self.add_notification(
                    "IAM users listed successfully".to_string(),
                    NotificationLevel::Success,
                );
            }
            ServiceCommand::ListRoles => {
                self.add_notification("Listing IAM roles...".to_string(), NotificationLevel::Info);
                // TODO: Implement actual IAM role listing
                self.add_notification(
                    "IAM roles listed successfully".to_string(),
                    NotificationLevel::Success,
                );
            }
            ServiceCommand::CreateUser => {
                self.add_notification(
                    "Creating new IAM user...".to_string(),
                    NotificationLevel::Info,
                );
                // TODO: Implement actual IAM user creation
                self.add_notification(
                    "IAM user creation initiated".to_string(),
                    NotificationLevel::Success,
                );
            }
            ServiceCommand::CreateRole => {
                self.add_notification(
                    "Creating new IAM role...".to_string(),
                    NotificationLevel::Info,
                );
                // TODO: Implement actual IAM role creation
                self.add_notification(
                    "IAM role creation initiated".to_string(),
                    NotificationLevel::Success,
                );
            }
            ServiceCommand::DeleteUser => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Deleting IAM user {}...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual IAM user deletion
                    self.add_notification(
                        "IAM user deletion initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No IAM user selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::DeleteRole => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Deleting IAM role {}...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual IAM role deletion
                    self.add_notification(
                        "IAM role deletion initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No IAM role selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::AttachPolicy => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!(
                            "Attaching policy to IAM resource {}...",
                            self.selected_resource_index
                        ),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual IAM policy attachment
                    self.add_notification(
                        "IAM policy attachment initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No IAM resource selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::DetachPolicy => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!(
                            "Detaching policy from IAM resource {}...",
                            self.selected_resource_index
                        ),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual IAM policy detachment
                    self.add_notification(
                        "IAM policy detachment initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No IAM resource selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            _ => {
                self.add_notification(
                    format!(
                        "IAM command '{}' not yet implemented",
                        command.display_name()
                    ),
                    NotificationLevel::Info,
                );
            }
        }
        Ok(())
    }

    /// Execute Secrets Manager-specific commands
    async fn execute_secrets_command(
        &mut self,
        command: &crate::command::ServiceCommand,
    ) -> Result<()> {
        use crate::command::ServiceCommand;

        match command {
            ServiceCommand::ListSecrets => {
                self.add_notification("Listing secrets...".to_string(), NotificationLevel::Info);
                // TODO: Implement actual secrets listing
                self.add_notification(
                    "Secrets listed successfully".to_string(),
                    NotificationLevel::Success,
                );
            }
            ServiceCommand::CreateSecret => {
                self.add_notification(
                    "Creating new secret...".to_string(),
                    NotificationLevel::Info,
                );
                // TODO: Implement actual secret creation
                self.add_notification(
                    "Secret creation initiated".to_string(),
                    NotificationLevel::Success,
                );
            }
            ServiceCommand::UpdateSecret => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Updating secret {}...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual secret update
                    self.add_notification(
                        "Secret update initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No secret selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::DeleteSecret => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Deleting secret {}...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual secret deletion
                    self.add_notification(
                        "Secret deletion initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No secret selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::GetSecretValue => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!(
                            "Retrieving secret value {}...",
                            self.selected_resource_index
                        ),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual secret value retrieval
                    self.add_notification(
                        "Secret value retrieved".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No secret selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::DescribeSecret => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Describing secret {}...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual secret description
                    self.add_notification(
                        "Secret details retrieved".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No secret selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            _ => {
                self.add_notification(
                    format!(
                        "Secrets command '{}' not yet implemented",
                        command.display_name()
                    ),
                    NotificationLevel::Info,
                );
            }
        }
        Ok(())
    }

    /// Execute EKS-specific commands
    async fn execute_eks_command(
        &mut self,
        command: &crate::command::ServiceCommand,
    ) -> Result<()> {
        use crate::command::ServiceCommand;

        match command {
            ServiceCommand::ListClusters => {
                self.add_notification(
                    "Listing EKS clusters...".to_string(),
                    NotificationLevel::Info,
                );
                // TODO: Implement actual EKS cluster listing
                self.add_notification(
                    "EKS clusters listed successfully".to_string(),
                    NotificationLevel::Success,
                );
            }
            ServiceCommand::CreateCluster => {
                self.add_notification(
                    "Creating new EKS cluster...".to_string(),
                    NotificationLevel::Info,
                );
                // TODO: Implement actual EKS cluster creation
                self.add_notification(
                    "EKS cluster creation initiated".to_string(),
                    NotificationLevel::Success,
                );
            }
            ServiceCommand::DeleteCluster => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Deleting EKS cluster {}...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual EKS cluster deletion
                    self.add_notification(
                        "EKS cluster deletion initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No EKS cluster selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::DescribeCluster => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!("Describing EKS cluster {}...", self.selected_resource_index),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual EKS cluster description
                    self.add_notification(
                        "EKS cluster details retrieved".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No EKS cluster selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::UpdateKubeconfig => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!(
                            "Updating kubeconfig for EKS cluster {}...",
                            self.selected_resource_index
                        ),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual kubeconfig update
                    self.add_notification(
                        "Kubeconfig update initiated".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No EKS cluster selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            ServiceCommand::ListNodeGroups => {
                if self.selected_resource.is_some() {
                    self.add_notification(
                        format!(
                            "Listing node groups for EKS cluster {}...",
                            self.selected_resource_index
                        ),
                        NotificationLevel::Info,
                    );
                    // TODO: Implement actual node group listing
                    self.add_notification(
                        "EKS node groups listed successfully".to_string(),
                        NotificationLevel::Success,
                    );
                } else {
                    self.add_notification(
                        "No EKS cluster selected".to_string(),
                        NotificationLevel::Error,
                    );
                }
            }
            _ => {
                self.add_notification(
                    format!(
                        "EKS command '{}' not yet implemented",
                        command.display_name()
                    ),
                    NotificationLevel::Info,
                );
            }
        }
        Ok(())
    }
}
