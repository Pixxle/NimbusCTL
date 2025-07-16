use crate::aws::types::ServiceType;
use crate::utils::error::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FavoriteResource {
    pub id: String,
    pub name: String,
    pub service_type: ServiceType,
    pub region: String,
    pub arn: String,
    pub tags: HashMap<String, String>,
    pub added_at: chrono::DateTime<chrono::Utc>,
    pub last_accessed: chrono::DateTime<chrono::Utc>,
    pub access_count: u32,
}

pub struct FavoritesManager {
    favorites: HashMap<String, FavoriteResource>,
    config_path: PathBuf,
}

impl FavoritesManager {
    pub fn new() -> Result<Self> {
        let config_dir = dirs::config_dir()
            .ok_or("Cannot find config directory")?
            .join("nimbus-ctl");

        let config_path = config_dir.join("favorites.json");

        let favorites = if config_path.exists() {
            let content = std::fs::read_to_string(&config_path)?;
            serde_json::from_str(&content)?
        } else {
            HashMap::new()
        };

        Ok(Self {
            favorites,
            config_path,
        })
    }

    pub fn add_favorite(&mut self, resource: FavoriteResource) -> Result<()> {
        self.favorites.insert(resource.id.clone(), resource);
        self.save()
    }

    pub fn remove_favorite(&mut self, resource_id: &str) -> Result<()> {
        self.favorites.remove(resource_id);
        self.save()
    }

    pub fn get_favorites(&self) -> Vec<&FavoriteResource> {
        let mut favorites: Vec<&FavoriteResource> = self.favorites.values().collect();
        favorites.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
        favorites
    }

    pub fn get_favorites_by_service(&self, service_type: ServiceType) -> Vec<&FavoriteResource> {
        let mut favorites: Vec<&FavoriteResource> = self
            .favorites
            .values()
            .filter(|f| f.service_type == service_type)
            .collect();
        favorites.sort_by(|a, b| b.last_accessed.cmp(&a.last_accessed));
        favorites
    }

    pub fn get_favorite(&self, resource_id: &str) -> Option<&FavoriteResource> {
        self.favorites.get(resource_id)
    }

    pub fn is_favorite(&self, resource_id: &str) -> bool {
        self.favorites.contains_key(resource_id)
    }

    pub fn update_access(&mut self, resource_id: &str) -> Result<()> {
        if let Some(favorite) = self.favorites.get_mut(resource_id) {
            favorite.last_accessed = chrono::Utc::now();
            favorite.access_count += 1;
            self.save()?;
        }
        Ok(())
    }

    pub fn get_most_accessed(&self, limit: usize) -> Vec<&FavoriteResource> {
        let mut favorites: Vec<&FavoriteResource> = self.favorites.values().collect();
        favorites.sort_by(|a, b| b.access_count.cmp(&a.access_count));
        favorites.into_iter().take(limit).collect()
    }

    pub fn get_recently_added(&self, limit: usize) -> Vec<&FavoriteResource> {
        let mut favorites: Vec<&FavoriteResource> = self.favorites.values().collect();
        favorites.sort_by(|a, b| b.added_at.cmp(&a.added_at));
        favorites.into_iter().take(limit).collect()
    }

    pub fn get_by_tag(&self, tag_key: &str, tag_value: Option<&str>) -> Vec<&FavoriteResource> {
        self.favorites
            .values()
            .filter(|f| {
                if let Some(value) = f.tags.get(tag_key) {
                    if let Some(expected_value) = tag_value {
                        value == expected_value
                    } else {
                        true
                    }
                } else {
                    false
                }
            })
            .collect()
    }

    pub fn search(&self, query: &str) -> Vec<&FavoriteResource> {
        let query_lower = query.to_lowercase();
        self.favorites
            .values()
            .filter(|f| {
                f.name.to_lowercase().contains(&query_lower)
                    || f.id.to_lowercase().contains(&query_lower)
                    || f.tags
                        .values()
                        .any(|v| v.to_lowercase().contains(&query_lower))
            })
            .collect()
    }

    pub fn count(&self) -> usize {
        self.favorites.len()
    }

    pub fn count_by_service(&self, service_type: ServiceType) -> usize {
        self.favorites
            .values()
            .filter(|f| f.service_type == service_type)
            .count()
    }

    fn save(&self) -> Result<()> {
        if let Some(parent) = self.config_path.parent() {
            std::fs::create_dir_all(parent)?;
        }

        let content = serde_json::to_string_pretty(&self.favorites)?;
        std::fs::write(&self.config_path, content)?;
        Ok(())
    }

    pub fn export_favorites(&self) -> Result<String> {
        let json = serde_json::to_string_pretty(&self.favorites)?;
        Ok(json)
    }

    pub fn import_favorites(&mut self, json: &str) -> Result<usize> {
        let imported: HashMap<String, FavoriteResource> = serde_json::from_str(json)?;
        let count = imported.len();

        for (id, favorite) in imported {
            self.favorites.insert(id, favorite);
        }

        self.save()?;
        Ok(count)
    }
}
