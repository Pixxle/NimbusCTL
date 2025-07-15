use crate::app::state::AppPage;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserConfig {
    pub aws: AwsConfig,
    pub display: DisplayConfig,
    pub behavior: BehaviorConfig,
    pub dashboard: DashboardConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AwsConfig {
    pub default_profile: String,
    pub default_region: String,
    pub auto_refresh_interval: u64,
    pub max_concurrent_requests: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DisplayConfig {
    pub theme: String,
    pub show_help_bar: bool,
    pub show_status_bar: bool,
    pub use_unicode_symbols: bool,
    pub max_table_rows: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BehaviorConfig {
    pub auto_refresh_resources: bool,
    pub confirm_destructive_actions: bool,
    pub remember_last_page: bool,
    pub save_favorites: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DashboardConfig {
    pub default_page: AppPage,
    pub enabled_widgets: Vec<String>,
    pub widget_positions: HashMap<String, (u16, u16)>,
    pub auto_refresh_dashboard: bool,
    pub dashboard_refresh_interval: u64,
    pub max_recent_items: usize,
    pub max_favorite_items: usize,
}

impl Default for UserConfig {
    fn default() -> Self {
        Self {
            aws: AwsConfig::default(),
            display: DisplayConfig::default(),
            behavior: BehaviorConfig::default(),
            dashboard: DashboardConfig::default(),
        }
    }
}

impl Default for AwsConfig {
    fn default() -> Self {
        Self {
            default_profile: "default".to_string(),
            default_region: "us-east-1".to_string(),
            auto_refresh_interval: 300,
            max_concurrent_requests: 10,
        }
    }
}

impl Default for DisplayConfig {
    fn default() -> Self {
        Self {
            theme: "default".to_string(),
            show_help_bar: true,
            show_status_bar: true,
            use_unicode_symbols: true,
            max_table_rows: 50,
        }
    }
}

impl Default for BehaviorConfig {
    fn default() -> Self {
        Self {
            auto_refresh_resources: true,
            confirm_destructive_actions: true,
            remember_last_page: true,
            save_favorites: true,
        }
    }
}

impl Default for DashboardConfig {
    fn default() -> Self {
        Self {
            default_page: AppPage::Dashboard,
            enabled_widgets: vec![
                "favorites".to_string(),
                "recent".to_string(),
                "quick_actions".to_string(),
                "region_overview".to_string(),
                "service_status".to_string(),
            ],
            widget_positions: HashMap::new(),
            auto_refresh_dashboard: true,
            dashboard_refresh_interval: 60,
            max_recent_items: 10,
            max_favorite_items: 10,
        }
    }
}

impl UserConfig {
    pub fn load() -> Result<Self, Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        
        if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let config: UserConfig = toml::from_str(&content)?;
            Ok(config)
        } else {
            let default_config = Self::default();
            default_config.save()?;
            Ok(default_config)
        }
    }
    
    pub fn save(&self) -> Result<(), Box<dyn std::error::Error>> {
        let config_path = Self::get_config_path()?;
        
        if let Some(parent) = config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let content = toml::to_string_pretty(self)?;
        std::fs::write(config_path, content)?;
        
        Ok(())
    }
    
    fn get_config_path() -> Result<PathBuf, Box<dyn std::error::Error>> {
        let config_dir = dirs::config_dir()
            .ok_or("Cannot find config directory")?
            .join("nimbus-ctl");
        
        Ok(config_dir.join("config.toml"))
    }
}