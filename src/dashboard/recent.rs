use crate::app::state::ActivityEntry;
use crate::aws::types::ServiceType;
use crate::utils::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::VecDeque;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RecentActivity {
    activities: VecDeque<ActivityEntry>,
    max_items: usize,
    config_path: PathBuf,
}

impl RecentActivity {
    pub fn new(max_items: usize) -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or("Cannot find config directory")?
            .join("nimbus-ctl");
        
        let config_path = config_dir.join("recent_activity.json");
        
        let activities = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            let loaded: Vec<ActivityEntry> = serde_json::from_str(&content)?;
            loaded.into_iter().collect()
        } else {
            VecDeque::new()
        };
        
        Ok(Self {
            activities,
            max_items,
            config_path,
        })
    }
    
    pub fn add_activity(&mut self, activity: ActivityEntry) -> Result<()> {
        // Remove any existing activity for the same resource
        self.activities.retain(|a| a.resource_id != activity.resource_id);
        
        // Add new activity to the front
        self.activities.push_front(activity);
        
        // Maintain max items limit
        while self.activities.len() > self.max_items {
            self.activities.pop_back();
        }
        
        self.save()
    }
    
    pub fn get_recent_activities(&self, limit: Option<usize>) -> Vec<&ActivityEntry> {
        let limit = limit.unwrap_or(self.max_items);
        self.activities.iter().take(limit).collect()
    }
    
    pub fn get_recent_by_service(&self, service_type: ServiceType, limit: Option<usize>) -> Vec<&ActivityEntry> {
        let limit = limit.unwrap_or(self.max_items);
        self.activities
            .iter()
            .filter(|a| a.service_type == service_type)
            .take(limit)
            .collect()
    }
    
    pub fn get_recent_by_region(&self, region: &str, limit: Option<usize>) -> Vec<&ActivityEntry> {
        let limit = limit.unwrap_or(self.max_items);
        self.activities
            .iter()
            .filter(|a| a.region == region)
            .take(limit)
            .collect()
    }
    
    pub fn get_recent_by_action(&self, action: &str, limit: Option<usize>) -> Vec<&ActivityEntry> {
        let limit = limit.unwrap_or(self.max_items);
        self.activities
            .iter()
            .filter(|a| a.action.contains(action))
            .take(limit)
            .collect()
    }
    
    pub fn get_recent_for_resource(&self, resource_id: &str) -> Option<&ActivityEntry> {
        self.activities.iter().find(|a| a.resource_id == resource_id)
    }
    
    pub fn remove_activity(&mut self, resource_id: &str) -> Result<()> {
        self.activities.retain(|a| a.resource_id != resource_id);
        self.save()
    }
    
    pub fn clear_activities(&mut self) -> Result<()> {
        self.activities.clear();
        self.save()
    }
    
    pub fn clear_old_activities(&mut self, max_age: chrono::Duration) -> Result<()> {
        let cutoff = chrono::Utc::now() - max_age;
        self.activities.retain(|a| a.timestamp > cutoff);
        self.save()
    }
    
    pub fn count(&self) -> usize {
        self.activities.len()
    }
    
    pub fn count_by_service(&self, service_type: ServiceType) -> usize {
        self.activities
            .iter()
            .filter(|a| a.service_type == service_type)
            .count()
    }
    
    pub fn search(&self, query: &str) -> Vec<&ActivityEntry> {
        let query_lower = query.to_lowercase();
        self.activities
            .iter()
            .filter(|a| {
                a.resource_name.to_lowercase().contains(&query_lower) ||
                a.resource_id.to_lowercase().contains(&query_lower) ||
                a.action.to_lowercase().contains(&query_lower)
            })
            .collect()
    }
    
    pub fn get_activity_summary(&self) -> ActivitySummary {
        let total_count = self.activities.len();
        let mut service_counts = std::collections::HashMap::new();
        let mut action_counts = std::collections::HashMap::new();
        
        for activity in &self.activities {
            *service_counts.entry(activity.service_type).or_insert(0) += 1;
            *action_counts.entry(activity.action.clone()).or_insert(0) += 1;
        }
        
        ActivitySummary {
            total_count,
            service_counts,
            action_counts,
        }
    }
    
    fn save(&self) -> Result<()> {
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }
        
        let activities_vec: Vec<&ActivityEntry> = self.activities.iter().collect();
        let content = serde_json::to_string_pretty(&activities_vec)?;
        std::fs::write(&self.config_path, content)?;
        Ok(())
    }
}

#[derive(Debug, Clone)]
pub struct ActivitySummary {
    pub total_count: usize,
    pub service_counts: std::collections::HashMap<ServiceType, usize>,
    pub action_counts: std::collections::HashMap<String, usize>,
}

// Helper functions for creating activity entries
impl ActivityEntry {
    pub fn new(
        action: String,
        resource_id: String,
        resource_name: String,
        service_type: ServiceType,
        region: String,
    ) -> Self {
        Self {
            timestamp: chrono::Utc::now(),
            action,
            resource_id,
            resource_name,
            service_type,
            region,
        }
    }
    
    pub fn created(
        resource_id: String,
        resource_name: String,
        service_type: ServiceType,
        region: String,
    ) -> Self {
        Self::new(
            "Created".to_string(),
            resource_id,
            resource_name,
            service_type,
            region,
        )
    }
    
    pub fn accessed(
        resource_id: String,
        resource_name: String,
        service_type: ServiceType,
        region: String,
    ) -> Self {
        Self::new(
            "Accessed".to_string(),
            resource_id,
            resource_name,
            service_type,
            region,
        )
    }
    
    pub fn modified(
        resource_id: String,
        resource_name: String,
        service_type: ServiceType,
        region: String,
    ) -> Self {
        Self::new(
            "Modified".to_string(),
            resource_id,
            resource_name,
            service_type,
            region,
        )
    }
    
    pub fn deleted(
        resource_id: String,
        resource_name: String,
        service_type: ServiceType,
        region: String,
    ) -> Self {
        Self::new(
            "Deleted".to_string(),
            resource_id,
            resource_name,
            service_type,
            region,
        )
    }
}