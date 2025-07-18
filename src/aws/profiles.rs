use crate::aws::types::{AwsProfile, CredentialSource, ProfileMetadata, ValidationStatus};
use crate::utils::error::Result;
use configparser::ini::Ini;
use std::collections::HashMap;
use std::env;
use std::path::PathBuf;

pub struct ProfileManager {
    profiles: HashMap<String, AwsProfile>,
    environment_profile: Option<AwsProfile>,
    credentials_path: PathBuf,
    config_path: PathBuf,
    profile_metadata: HashMap<String, ProfileMetadata>,
}

impl ProfileManager {
    pub fn new() -> Result<Self> {
        let home = dirs::home_dir().ok_or("Cannot find home directory")?;
        let aws_dir = home.join(".aws");

        let credentials_path = aws_dir.join("credentials");
        let config_path = aws_dir.join("config");

        let mut manager = Self {
            profiles: HashMap::new(),
            environment_profile: None,
            credentials_path,
            config_path,
            profile_metadata: HashMap::new(),
        };

        manager.load_all_profiles()?;
        Ok(manager)
    }

    pub fn load_all_profiles(&mut self) -> Result<()> {
        // Clear existing profiles
        self.profiles.clear();
        self.profile_metadata.clear();

        // Load profiles from credentials file
        self.load_credentials_file()?;

        // Load profiles from config file
        self.load_config_file()?;

        // Detect environment credentials
        self.detect_environment_credentials()?;

        Ok(())
    }

    fn load_credentials_file(&mut self) -> Result<()> {
        if !self.credentials_path.exists() {
            return Ok(());
        }

        let mut config = Ini::new();
        config
            .load(&self.credentials_path)
            .map_err(|e| format!("Failed to load credentials file: {}", e))?;

        for section_name in config.sections() {
            if let Some(section) = config.get_map_ref().get(&section_name) {
                let profile_name = section_name.clone();

                let access_key_id = section.get("aws_access_key_id").and_then(|s| s.clone());
                let secret_access_key =
                    section.get("aws_secret_access_key").and_then(|s| s.clone());
                let session_token = section.get("aws_session_token").and_then(|s| s.clone());

                // Only create profile if we have at least access key and secret
                if access_key_id.is_some() && secret_access_key.is_some() {
                    let profile = AwsProfile {
                        name: profile_name.clone(),
                        region: section.get("region").and_then(|s| s.clone()),
                        access_key_id,
                        secret_access_key,
                        session_token,
                        role_arn: None,
                        source_profile: None,
                        mfa_serial: None,
                        external_id: None,
                        credential_source: CredentialSource::ConfigFile(profile_name.clone()),
                    };

                    // Initialize metadata
                    let metadata = ProfileMetadata {
                        account_id: None,
                        user_arn: None,
                        role_arn: None,
                        mfa_required: false,
                        session_duration: None,
                        last_validated: None,
                        validation_status: ValidationStatus::Unknown,
                    };

                    self.profiles.insert(profile_name.clone(), profile);
                    self.profile_metadata.insert(profile_name, metadata);
                }
            }
        }

        Ok(())
    }

    fn load_config_file(&mut self) -> Result<()> {
        if !self.config_path.exists() {
            return Ok(());
        }

        let mut config = Ini::new();
        config
            .load(&self.config_path)
            .map_err(|e| format!("Failed to load config file: {}", e))?;

        for section_name in config.sections() {
            if let Some(section) = config.get_map_ref().get(&section_name) {
                // Handle both "default" and "profile xxx" sections
                let profile_name = if section_name == "default" {
                    "default".to_string()
                } else if section_name.starts_with("profile ") {
                    section_name.strip_prefix("profile ").unwrap().to_string()
                } else {
                    continue;
                };

                // Get or create profile (might already exist from credentials file)
                let mut profile = self
                    .profiles
                    .get(&profile_name)
                    .cloned()
                    .unwrap_or_else(|| AwsProfile {
                        name: profile_name.clone(),
                        region: None,
                        access_key_id: None,
                        secret_access_key: None,
                        session_token: None,
                        role_arn: None,
                        source_profile: None,
                        mfa_serial: None,
                        external_id: None,
                        credential_source: CredentialSource::ConfigFile(profile_name.clone()),
                    });

                // Update profile with config file data
                if let Some(Some(region)) = section.get("region") {
                    profile.region = Some(region.clone());
                }
                if let Some(Some(role_arn)) = section.get("role_arn") {
                    profile.role_arn = Some(role_arn.clone());
                }
                if let Some(Some(source_profile)) = section.get("source_profile") {
                    profile.source_profile = Some(source_profile.clone());
                }
                if let Some(Some(mfa_serial)) = section.get("mfa_serial") {
                    profile.mfa_serial = Some(mfa_serial.clone());
                }
                if let Some(Some(external_id)) = section.get("external_id") {
                    profile.external_id = Some(external_id.clone());
                }

                // Initialize or update metadata
                let mut metadata = self
                    .profile_metadata
                    .get(&profile_name)
                    .cloned()
                    .unwrap_or_else(|| ProfileMetadata {
                        account_id: None,
                        user_arn: None,
                        role_arn: profile.role_arn.clone(),
                        mfa_required: profile.mfa_serial.is_some(),
                        session_duration: None,
                        last_validated: None,
                        validation_status: ValidationStatus::Unknown,
                    });

                // Update metadata with config info
                metadata.role_arn = profile.role_arn.clone();
                metadata.mfa_required = profile.mfa_serial.is_some();

                self.profiles.insert(profile_name.clone(), profile);
                self.profile_metadata.insert(profile_name, metadata);
            }
        }

        Ok(())
    }

    pub fn detect_environment_credentials(&mut self) -> Result<()> {
        let access_key_id = env::var("AWS_ACCESS_KEY_ID").ok();
        let secret_access_key = env::var("AWS_SECRET_ACCESS_KEY").ok();
        let session_token = env::var("AWS_SESSION_TOKEN").ok();
        let region = env::var("AWS_DEFAULT_REGION").ok();

        // Only create environment profile if we have at least access key and secret
        if let (Some(access_key), Some(secret_key)) = (access_key_id, secret_access_key) {
            let profile = AwsProfile {
                name: "Environment Variables".to_string(),
                region,
                access_key_id: Some(access_key),
                secret_access_key: Some(secret_key),
                session_token,
                role_arn: None,
                source_profile: None,
                mfa_serial: None,
                external_id: None,
                credential_source: CredentialSource::Environment,
            };

            let metadata = ProfileMetadata {
                account_id: None,
                user_arn: None,
                role_arn: None,
                mfa_required: false,
                session_duration: None,
                last_validated: None,
                validation_status: ValidationStatus::Unknown,
            };

            self.environment_profile = Some(profile.clone());
            self.profiles
                .insert("Environment Variables".to_string(), profile);
            self.profile_metadata
                .insert("Environment Variables".to_string(), metadata);
        }

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

    pub fn get_profile_with_metadata(&self, name: &str) -> Option<(&AwsProfile, &ProfileMetadata)> {
        if let (Some(profile), Some(metadata)) =
            (self.profiles.get(name), self.profile_metadata.get(name))
        {
            Some((profile, metadata))
        } else {
            None
        }
    }

    pub fn get_available_credential_sources(&self) -> Vec<CredentialSource> {
        let mut sources = Vec::new();

        // Add environment credentials if available
        if self.environment_profile.is_some() {
            sources.push(CredentialSource::Environment);
        }

        // Add file-based profiles
        for profile in self.profiles.values() {
            if let CredentialSource::ConfigFile(profile_name) = &profile.credential_source {
                if profile_name != "Environment Variables" {
                    sources.push(CredentialSource::ConfigFile(profile_name.clone()));
                }
            }
        }

        // Remove duplicates
        sources.sort_by(|a, b| match (a, b) {
            (CredentialSource::Environment, CredentialSource::Environment) => {
                std::cmp::Ordering::Equal
            }
            (CredentialSource::Environment, _) => std::cmp::Ordering::Less,
            (_, CredentialSource::Environment) => std::cmp::Ordering::Greater,
            (CredentialSource::ConfigFile(a), CredentialSource::ConfigFile(b)) => a.cmp(b),
        });
        sources.dedup();

        sources
    }

    pub fn get_profile_metadata(&self, name: &str) -> Option<&ProfileMetadata> {
        self.profile_metadata.get(name)
    }

    pub fn update_profile_metadata(&mut self, name: &str, metadata: ProfileMetadata) {
        self.profile_metadata.insert(name.to_string(), metadata);
    }
}
