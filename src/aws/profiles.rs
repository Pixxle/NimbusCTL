use crate::aws::types::AwsProfile;
use crate::utils::error::Result;
use configparser::ini::Ini;
use std::collections::HashMap;
use std::path::PathBuf;

pub struct ProfileManager {
    profiles: HashMap<String, AwsProfile>,
    credentials_path: PathBuf,
    config_path: PathBuf,
}

impl ProfileManager {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().ok_or("Cannot find home directory")?;
        let aws_dir = home.join(".aws");
        
        let credentials_path = aws_dir.join("credentials");
        let config_path = aws_dir.join("config");
        
        let mut manager = Self {
            profiles: HashMap::new(),
            credentials_path,
            config_path,
        };
        
        manager.load_profiles()?;
        Ok(manager)
    }
    
    fn load_profiles(&mut self) -> Result<()> {
        // For Phase 1, just load a default profile
        // Real profile loading will be implemented in Phase 2
        self.profiles.insert("default".to_string(), AwsProfile {
            name: "default".to_string(),
            region: Some("us-east-1".to_string()),
            access_key_id: None,
            secret_access_key: None,
            session_token: None,
            role_arn: None,
            source_profile: None,
        });
        
        Ok(())
    }
    
    pub fn get_profiles(&self) -> Vec<&AwsProfile> {
        self.profiles.values().collect()
    }
    
    pub fn get_profile(&self, name: &str) -> Option<&AwsProfile> {
        self.profiles.get(name)
    }
    
    pub fn profile_exists(&self, name: &str) -> bool {
        self.profiles.contains_key(name)
    }
    
    pub fn get_default_profile(&self) -> Option<&AwsProfile> {
        self.profiles.get("default")
    }
}