use crate::config::user_config::UserConfig;
use crate::utils::error::Result;
use std::path::PathBuf;

pub struct AppConfig {
    pub user_config: UserConfig,
    pub config_path: PathBuf,
}

impl AppConfig {
    pub fn new() -> Result<Self> {
        let user_config = UserConfig::load()?;
        let config_path = Self::get_config_path()?;

        Ok(Self {
            user_config,
            config_path,
        })
    }

    pub fn save(&self) -> Result<()> {
        self.user_config.save()?;
        Ok(())
    }

    pub fn reset_to_defaults(&mut self) -> Result<()> {
        self.user_config = UserConfig::default();
        self.save()?;
        Ok(())
    }

    fn get_config_path() -> Result<PathBuf> {
        let config_dir = dirs::config_dir()
            .ok_or("Cannot find config directory")?
            .join("nimbus-ctl");

        Ok(config_dir.join("config.toml"))
    }
}
