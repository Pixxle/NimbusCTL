use crate::config::user_config::UserConfig;
use crate::utils::error::Result;

pub struct SettingsManager {
    config: UserConfig,
}

impl SettingsManager {
    pub fn new() -> Result<Self> {
        let config = UserConfig::load()?;
        Ok(Self { config })
    }
    
    pub fn get_config(&self) -> &UserConfig {
        &self.config
    }
    
    pub fn get_config_mut(&mut self) -> &mut UserConfig {
        &mut self.config
    }
    
    pub fn save(&self) -> Result<()> {
        self.config.save()?;
        Ok(())
    }
    
    pub fn reset_to_defaults(&mut self) -> Result<()> {
        self.config = UserConfig::default();
        self.save()?;
        Ok(())
    }
    
    pub fn update_aws_profile(&mut self, profile: String) -> Result<()> {
        self.config.aws.default_profile = profile;
        self.save()?;
        Ok(())
    }
    
    pub fn update_aws_region(&mut self, region: String) -> Result<()> {
        self.config.aws.default_region = region;
        self.save()?;
        Ok(())
    }
    
    pub fn update_theme(&mut self, theme: String) -> Result<()> {
        self.config.display.theme = theme;
        self.save()?;
        Ok(())
    }
    
    pub fn toggle_help_bar(&mut self) -> Result<()> {
        self.config.display.show_help_bar = !self.config.display.show_help_bar;
        self.save()?;
        Ok(())
    }
    
    pub fn toggle_status_bar(&mut self) -> Result<()> {
        self.config.display.show_status_bar = !self.config.display.show_status_bar;
        self.save()?;
        Ok(())
    }
    
    pub fn toggle_unicode_symbols(&mut self) -> Result<()> {
        self.config.display.use_unicode_symbols = !self.config.display.use_unicode_symbols;
        self.save()?;
        Ok(())
    }
    
    pub fn toggle_auto_refresh(&mut self) -> Result<()> {
        self.config.behavior.auto_refresh_resources = !self.config.behavior.auto_refresh_resources;
        self.save()?;
        Ok(())
    }
    
    pub fn toggle_confirm_actions(&mut self) -> Result<()> {
        self.config.behavior.confirm_destructive_actions = !self.config.behavior.confirm_destructive_actions;
        self.save()?;
        Ok(())
    }
    
    pub fn set_refresh_interval(&mut self, interval: u64) -> Result<()> {
        self.config.aws.auto_refresh_interval = interval;
        self.save()?;
        Ok(())
    }
    
    pub fn set_dashboard_refresh_interval(&mut self, interval: u64) -> Result<()> {
        self.config.dashboard.dashboard_refresh_interval = interval;
        self.save()?;
        Ok(())
    }
    
    pub fn set_max_recent_items(&mut self, max: usize) -> Result<()> {
        self.config.dashboard.max_recent_items = max;
        self.save()?;
        Ok(())
    }
    
    pub fn set_max_favorite_items(&mut self, max: usize) -> Result<()> {
        self.config.dashboard.max_favorite_items = max;
        self.save()?;
        Ok(())
    }
}