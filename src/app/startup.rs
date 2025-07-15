use crate::app::config::AppConfig;
use crate::aws::profiles::ProfileManager;
use crate::utils::error::Result;
use tracing::info;

pub struct StartupManager;

impl StartupManager {
    pub async fn initialize() -> Result<()> {
        // Initialize logging
        Self::init_logging()?;
        
        // Load configuration
        let config = AppConfig::new()?;
        info!("Configuration loaded successfully");
        
        // Initialize AWS profile manager
        let profile_manager = ProfileManager::new()?;
        info!("AWS profiles loaded: {}", profile_manager.get_profiles().len());
        
        // Validate AWS credentials if available
        Self::validate_aws_setup(&profile_manager).await?;
        
        info!("Application startup completed successfully");
        Ok(())
    }
    
    fn init_logging() -> Result<()> {
        tracing_subscriber::fmt()
            .init();
        
        info!("Logging initialized");
        Ok(())
    }
    
    async fn validate_aws_setup(profile_manager: &ProfileManager) -> Result<()> {
        let profiles = profile_manager.get_profiles();
        
        if profiles.is_empty() {
            tracing::warn!("No AWS profiles found. Please configure AWS credentials.");
            return Ok(());
        }
        
        info!("Found {} AWS profiles", profiles.len());
        
        // Check if default profile exists
        if let Some(default_profile) = profile_manager.get_default_profile() {
            info!("Default profile found: {}", default_profile.name);
        } else {
            tracing::warn!("No default AWS profile found");
        }
        
        Ok(())
    }
    
    pub fn create_config_directories() -> Result<()> {
        let config_dir = dirs::config_dir()
            .ok_or("Cannot find config directory")?
            .join("nimbus-ctl");
        
        std::fs::create_dir_all(&config_dir)?;
        info!("Configuration directory created: {:?}", config_dir);
        
        Ok(())
    }
}