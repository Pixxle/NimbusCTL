use crate::utils::error::Result;
use std::collections::HashMap;

pub struct MultiRegionAwsClients {
    current_region: String,
    current_profile: String,
}

pub struct RegionClients {
    // AWS clients will be added back in Phase 2
    pub region: String,
}

impl MultiRegionAwsClients {
    pub async fn new(profile: &str, region: &str) -> Result<Self> {
        Ok(Self {
            current_region: region.to_string(),
            current_profile: profile.to_string(),
        })
    }
    
    pub async fn switch_region(&mut self, region: &str) -> Result<()> {
        self.current_region = region.to_string();
        Ok(())
    }
    
    pub async fn switch_profile(&mut self, profile: &str) -> Result<()> {
        self.current_profile = profile.to_string();
        Ok(())
    }
    
    pub fn get_current_clients(&self) -> Option<RegionClients> {
        Some(RegionClients {
            region: self.current_region.clone(),
        })
    }
    
    pub fn get_clients_for_region(&self, region: &str) -> Option<RegionClients> {
        Some(RegionClients {
            region: region.to_string(),
        })
    }
    
    pub fn current_region(&self) -> &str {
        &self.current_region
    }
    
    pub fn current_profile(&self) -> &str {
        &self.current_profile
    }
}