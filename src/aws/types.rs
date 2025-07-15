use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub type ResourceId = String;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceType {
    EC2,
    S3,
    RDS,
    IAM,
    Secrets,
    EKS,
}

impl ServiceType {
    pub fn all() -> Vec<ServiceType> {
        vec![
            ServiceType::EC2,
            ServiceType::S3,
            ServiceType::RDS,
            ServiceType::IAM,
            ServiceType::Secrets,
            ServiceType::EKS,
        ]
    }

    pub fn display_name(&self) -> &'static str {
        match self {
            ServiceType::EC2 => "EC2",
            ServiceType::S3 => "S3",
            ServiceType::RDS => "RDS",
            ServiceType::IAM => "IAM",
            ServiceType::Secrets => "Secrets Manager",
            ServiceType::EKS => "EKS",
        }
    }

    pub fn icon(&self) -> &'static str {
        match self {
            ServiceType::EC2 => "💻",
            ServiceType::S3 => "🪣",
            ServiceType::RDS => "🗄️",
            ServiceType::IAM => "👤",
            ServiceType::Secrets => "🔐",
            ServiceType::EKS => "⚙️",
        }
    }

    pub fn from_arn(arn: &str) -> crate::utils::error::Result<ServiceType> {
        let parts: Vec<&str> = arn.split(':').collect();
        if parts.len() >= 3 {
            match parts[2] {
                "ec2" => Ok(ServiceType::EC2),
                "s3" => Ok(ServiceType::S3),
                "rds" => Ok(ServiceType::RDS),
                "iam" => Ok(ServiceType::IAM),
                "secretsmanager" => Ok(ServiceType::Secrets),
                "eks" => Ok(ServiceType::EKS),
                _ => Err(crate::utils::error::AppError::Parse(format!(
                    "Unknown service type in ARN: {}",
                    arn
                ))),
            }
        } else {
            Err(crate::utils::error::AppError::Parse(format!(
                "Invalid ARN format: {}",
                arn
            )))
        }
    }
}

#[derive(Debug, Clone)]
pub struct AwsProfile {
    pub name: String,
    pub region: Option<String>,
    pub access_key_id: Option<String>,
    pub secret_access_key: Option<String>,
    pub session_token: Option<String>,
    pub role_arn: Option<String>,
    pub source_profile: Option<String>,
}

#[derive(Debug, Clone)]
pub struct AwsRegion {
    pub name: String,
    pub display_name: String,
}

#[derive(Debug, Clone)]
pub struct ResourceTag {
    pub key: String,
    pub value: String,
}

#[derive(Debug, Clone)]
pub struct Resource {
    pub id: ResourceId,
    pub name: String,
    pub service_type: ServiceType,
    pub region: String,
    pub arn: String,
    pub state: String,
    pub tags: HashMap<String, String>,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub last_modified: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone)]
pub struct TaggedResource {
    pub arn: String,
    pub service_type: ServiceType,
    pub resource_id: String,
    pub resource_name: Option<String>,
    pub tags: Vec<ResourceTag>,
    pub region: String,
}

#[derive(Debug, Clone)]
pub struct TagFilter {
    pub key: String,
    pub value: Option<String>,
}

#[derive(Debug, Clone)]
pub enum ActionType {
    Create,
    Read,
    Update,
    Delete,
    Start,
    Stop,
    Restart,
}

// Service-specific resource types
#[derive(Debug, Clone)]
pub struct Ec2Instance {
    pub instance_id: String,
    pub instance_type: String,
    pub state: String,
    pub availability_zone: String,
    pub public_ip: Option<String>,
    pub private_ip: Option<String>,
    pub vpc_id: Option<String>,
    pub subnet_id: Option<String>,
    pub security_groups: Vec<String>,
    pub launch_time: Option<chrono::DateTime<chrono::Utc>>,
}

#[derive(Debug, Clone)]
pub struct S3Bucket {
    pub name: String,
    pub region: String,
    pub creation_date: Option<chrono::DateTime<chrono::Utc>>,
    pub versioning: bool,
    pub encryption: bool,
    pub public_read: bool,
    pub size: Option<u64>,
    pub object_count: Option<u64>,
}

#[derive(Debug, Clone)]
pub struct RdsInstance {
    pub db_instance_identifier: String,
    pub db_instance_class: String,
    pub engine: String,
    pub engine_version: String,
    pub db_instance_status: String,
    pub allocated_storage: i32,
    pub availability_zone: String,
    pub endpoint: Option<String>,
    pub port: Option<i32>,
    pub vpc_security_groups: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct IamUser {
    pub user_name: String,
    pub user_id: String,
    pub arn: String,
    pub path: String,
    pub create_date: Option<chrono::DateTime<chrono::Utc>>,
    pub password_last_used: Option<chrono::DateTime<chrono::Utc>>,
    pub attached_policies: Vec<String>,
    pub groups: Vec<String>,
}

#[derive(Debug, Clone)]
pub struct Secret {
    pub name: String,
    pub arn: String,
    pub description: Option<String>,
    pub created_date: Option<chrono::DateTime<chrono::Utc>>,
    pub last_accessed_date: Option<chrono::DateTime<chrono::Utc>>,
    pub last_changed_date: Option<chrono::DateTime<chrono::Utc>>,
    pub version_id: Option<String>,
}

#[derive(Debug, Clone)]
pub struct EksCluster {
    pub name: String,
    pub arn: String,
    pub version: Option<String>,
    pub endpoint: Option<String>,
    pub role_arn: Option<String>,
    pub status: String,
    pub created_at: Option<chrono::DateTime<chrono::Utc>>,
    pub platform_version: Option<String>,
    pub vpc_config: Option<String>,
}
