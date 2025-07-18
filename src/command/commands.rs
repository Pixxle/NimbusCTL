use crate::app::state::AppPage;
use crate::aws::types::{ResourceId, ServiceType};
use serde::{Deserialize, Serialize};

/// Core command structure that represents an executable action in the command palette
#[derive(Debug, Clone)]
pub struct Command {
    /// Unique identifier for the command
    pub id: String,
    /// Display name shown in the command palette
    pub name: String,
    /// Description shown as help text
    pub description: String,
    /// Category for grouping commands
    pub category: CommandCategory,
    /// The action to execute when the command is selected
    pub action: CommandAction,
    /// Icon to display next to the command
    pub icon: String,
    /// Keywords for fuzzy search matching
    pub keywords: Vec<String>,
    /// Whether the command is currently enabled
    pub enabled: bool,
    /// Context requirements that must be met for this command to be available
    pub context_requirements: Vec<ContextRequirement>,
}

impl Command {
    /// Create a new command with the given parameters
    pub fn new(
        id: String,
        name: String,
        description: String,
        category: CommandCategory,
        action: CommandAction,
        icon: String,
    ) -> Self {
        Self {
            id,
            name,
            description,
            category,
            action,
            icon,
            keywords: Vec::new(),
            enabled: true,
            context_requirements: Vec::new(),
        }
    }

    /// Add keywords for fuzzy search
    pub fn with_keywords(mut self, keywords: Vec<String>) -> Self {
        self.keywords = keywords;
        self
    }

    /// Add context requirements
    pub fn with_context_requirements(mut self, requirements: Vec<ContextRequirement>) -> Self {
        self.context_requirements = requirements;
        self
    }

    /// Set enabled state
    pub fn with_enabled(mut self, enabled: bool) -> Self {
        self.enabled = enabled;
        self
    }
}

/// Categories for organizing commands in the palette
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum CommandCategory {
    /// Navigation commands (go to dashboard, services, etc.)
    Navigation,
    /// Profile switching commands
    Profile,
    /// Region switching commands
    Region,
    /// Service-specific commands
    Service(ServiceType),
    /// General application commands (help, settings, etc.)
    General,
}

impl CommandCategory {
    /// Get display name for the category
    pub fn display_name(&self) -> &'static str {
        match self {
            CommandCategory::Navigation => "Navigation",
            CommandCategory::Profile => "Profile",
            CommandCategory::Region => "Region",
            CommandCategory::Service(_) => "Service",
            CommandCategory::General => "General",
        }
    }

    /// Get icon for the category
    pub fn icon(&self) -> &'static str {
        match self {
            CommandCategory::Navigation => "🧭",
            CommandCategory::Profile => "👤",
            CommandCategory::Region => "🌍",
            CommandCategory::Service(service) => service.icon(),
            CommandCategory::General => "⚙️",
        }
    }
}

/// Actions that can be executed by commands
#[derive(Debug, Clone)]
pub enum CommandAction {
    /// Switch to a specific AWS profile
    SwitchProfile(String),
    /// Switch to a specific AWS region
    SwitchRegion(String),
    /// Navigate to a specific service page
    NavigateToService(ServiceType),
    /// Navigate to a specific page
    NavigateToPage(AppPage),
    /// Execute a service-specific command
    ExecuteServiceCommand(ServiceType, ServiceCommand),
    /// Show help dialog
    ShowHelp,
    /// Open settings page
    OpenSettings,
    /// Toggle a UI element
    ToggleUI(UIElement),
}

/// UI elements that can be toggled
#[derive(Debug, Clone)]
pub enum UIElement {
    ProfileSelector,
    RegionSelector,
    Help,
    Settings,
}

/// Service-specific commands that can be executed
#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ServiceCommand {
    // EC2 Commands
    StartInstance,
    StopInstance,
    RebootInstance,
    TerminateInstance,
    CreateInstance,
    DescribeInstance,
    ListInstances,

    // S3 Commands
    CreateBucket,
    DeleteBucket,
    ListObjects,
    UploadObject,
    DownloadObject,
    ListBuckets,
    GetBucketInfo,

    // RDS Commands
    StartDatabase,
    StopDatabase,
    RebootDatabase,
    CreateSnapshot,
    RestoreSnapshot,
    ListDatabases,
    DescribeDatabase,

    // IAM Commands
    CreateUser,
    DeleteUser,
    CreateRole,
    DeleteRole,
    AttachPolicy,
    DetachPolicy,
    ListUsers,
    ListRoles,

    // Secrets Manager Commands
    CreateSecret,
    UpdateSecret,
    DeleteSecret,
    GetSecretValue,
    ListSecrets,
    DescribeSecret,

    // EKS Commands
    DescribeCluster,
    UpdateKubeconfig,
    ListNodeGroups,
    ListClusters,
    CreateCluster,
    DeleteCluster,
}

impl ServiceCommand {
    /// Get display name for the service command
    pub fn display_name(&self) -> &'static str {
        match self {
            // EC2 Commands
            ServiceCommand::StartInstance => "Start Instance",
            ServiceCommand::StopInstance => "Stop Instance",
            ServiceCommand::RebootInstance => "Reboot Instance",
            ServiceCommand::TerminateInstance => "Terminate Instance",
            ServiceCommand::CreateInstance => "Create Instance",
            ServiceCommand::DescribeInstance => "Describe Instance",
            ServiceCommand::ListInstances => "List Instances",

            // S3 Commands
            ServiceCommand::CreateBucket => "Create Bucket",
            ServiceCommand::DeleteBucket => "Delete Bucket",
            ServiceCommand::ListObjects => "List Objects",
            ServiceCommand::UploadObject => "Upload Object",
            ServiceCommand::DownloadObject => "Download Object",
            ServiceCommand::ListBuckets => "List Buckets",
            ServiceCommand::GetBucketInfo => "Get Bucket Info",

            // RDS Commands
            ServiceCommand::StartDatabase => "Start Database",
            ServiceCommand::StopDatabase => "Stop Database",
            ServiceCommand::RebootDatabase => "Reboot Database",
            ServiceCommand::CreateSnapshot => "Create Snapshot",
            ServiceCommand::RestoreSnapshot => "Restore Snapshot",
            ServiceCommand::ListDatabases => "List Databases",
            ServiceCommand::DescribeDatabase => "Describe Database",

            // IAM Commands
            ServiceCommand::CreateUser => "Create User",
            ServiceCommand::DeleteUser => "Delete User",
            ServiceCommand::CreateRole => "Create Role",
            ServiceCommand::DeleteRole => "Delete Role",
            ServiceCommand::AttachPolicy => "Attach Policy",
            ServiceCommand::DetachPolicy => "Detach Policy",
            ServiceCommand::ListUsers => "List Users",
            ServiceCommand::ListRoles => "List Roles",

            // Secrets Manager Commands
            ServiceCommand::CreateSecret => "Create Secret",
            ServiceCommand::UpdateSecret => "Update Secret",
            ServiceCommand::DeleteSecret => "Delete Secret",
            ServiceCommand::GetSecretValue => "Get Secret Value",
            ServiceCommand::ListSecrets => "List Secrets",
            ServiceCommand::DescribeSecret => "Describe Secret",

            // EKS Commands
            ServiceCommand::DescribeCluster => "Describe Cluster",
            ServiceCommand::UpdateKubeconfig => "Update Kubeconfig",
            ServiceCommand::ListNodeGroups => "List Node Groups",
            ServiceCommand::ListClusters => "List Clusters",
            ServiceCommand::CreateCluster => "Create Cluster",
            ServiceCommand::DeleteCluster => "Delete Cluster",
        }
    }

    /// Get description for the service command
    pub fn description(&self) -> &'static str {
        match self {
            // EC2 Commands
            ServiceCommand::StartInstance => "Start the selected EC2 instance",
            ServiceCommand::StopInstance => "Stop the selected EC2 instance",
            ServiceCommand::RebootInstance => "Reboot the selected EC2 instance",
            ServiceCommand::TerminateInstance => "Terminate the selected EC2 instance",
            ServiceCommand::CreateInstance => "Launch a new EC2 instance",
            ServiceCommand::DescribeInstance => "Show details of the selected instance",
            ServiceCommand::ListInstances => "List all EC2 instances",

            // S3 Commands
            ServiceCommand::CreateBucket => "Create a new S3 bucket",
            ServiceCommand::DeleteBucket => "Delete the selected S3 bucket",
            ServiceCommand::ListObjects => "List objects in the selected bucket",
            ServiceCommand::UploadObject => "Upload an object to the selected bucket",
            ServiceCommand::DownloadObject => "Download the selected object",
            ServiceCommand::ListBuckets => "List all S3 buckets",
            ServiceCommand::GetBucketInfo => "Show details of the selected bucket",

            // RDS Commands
            ServiceCommand::StartDatabase => "Start the selected RDS instance",
            ServiceCommand::StopDatabase => "Stop the selected RDS instance",
            ServiceCommand::RebootDatabase => "Reboot the selected RDS instance",
            ServiceCommand::CreateSnapshot => "Create a snapshot of the selected database",
            ServiceCommand::RestoreSnapshot => "Restore database from snapshot",
            ServiceCommand::ListDatabases => "List all RDS instances",
            ServiceCommand::DescribeDatabase => "Show details of the selected database",

            // IAM Commands
            ServiceCommand::CreateUser => "Create a new IAM user",
            ServiceCommand::DeleteUser => "Delete the selected IAM user",
            ServiceCommand::CreateRole => "Create a new IAM role",
            ServiceCommand::DeleteRole => "Delete the selected IAM role",
            ServiceCommand::AttachPolicy => "Attach policy to user or role",
            ServiceCommand::DetachPolicy => "Detach policy from user or role",
            ServiceCommand::ListUsers => "List all IAM users",
            ServiceCommand::ListRoles => "List all IAM roles",

            // Secrets Manager Commands
            ServiceCommand::CreateSecret => "Create a new secret",
            ServiceCommand::UpdateSecret => "Update the selected secret",
            ServiceCommand::DeleteSecret => "Delete the selected secret",
            ServiceCommand::GetSecretValue => "Retrieve the secret value",
            ServiceCommand::ListSecrets => "List all secrets",
            ServiceCommand::DescribeSecret => "Show details of the selected secret",

            // EKS Commands
            ServiceCommand::DescribeCluster => "Show details of the selected cluster",
            ServiceCommand::UpdateKubeconfig => "Update kubeconfig for the cluster",
            ServiceCommand::ListNodeGroups => "List node groups in the cluster",
            ServiceCommand::ListClusters => "List all EKS clusters",
            ServiceCommand::CreateCluster => "Create a new EKS cluster",
            ServiceCommand::DeleteCluster => "Delete the selected EKS cluster",
        }
    }

    /// Get the service type this command belongs to
    pub fn service_type(&self) -> ServiceType {
        match self {
            ServiceCommand::StartInstance
            | ServiceCommand::StopInstance
            | ServiceCommand::RebootInstance
            | ServiceCommand::TerminateInstance
            | ServiceCommand::CreateInstance
            | ServiceCommand::DescribeInstance
            | ServiceCommand::ListInstances => ServiceType::EC2,

            ServiceCommand::CreateBucket
            | ServiceCommand::DeleteBucket
            | ServiceCommand::ListObjects
            | ServiceCommand::UploadObject
            | ServiceCommand::DownloadObject
            | ServiceCommand::ListBuckets
            | ServiceCommand::GetBucketInfo => ServiceType::S3,

            ServiceCommand::StartDatabase
            | ServiceCommand::StopDatabase
            | ServiceCommand::RebootDatabase
            | ServiceCommand::CreateSnapshot
            | ServiceCommand::RestoreSnapshot
            | ServiceCommand::ListDatabases
            | ServiceCommand::DescribeDatabase => ServiceType::RDS,

            ServiceCommand::CreateUser
            | ServiceCommand::DeleteUser
            | ServiceCommand::CreateRole
            | ServiceCommand::DeleteRole
            | ServiceCommand::AttachPolicy
            | ServiceCommand::DetachPolicy
            | ServiceCommand::ListUsers
            | ServiceCommand::ListRoles => ServiceType::IAM,

            ServiceCommand::CreateSecret
            | ServiceCommand::UpdateSecret
            | ServiceCommand::DeleteSecret
            | ServiceCommand::GetSecretValue
            | ServiceCommand::ListSecrets
            | ServiceCommand::DescribeSecret => ServiceType::Secrets,

            ServiceCommand::DescribeCluster
            | ServiceCommand::UpdateKubeconfig
            | ServiceCommand::ListNodeGroups
            | ServiceCommand::ListClusters
            | ServiceCommand::CreateCluster
            | ServiceCommand::DeleteCluster => ServiceType::EKS,
        }
    }

    /// Check if this command requires a resource to be selected
    pub fn requires_resource_selection(&self) -> bool {
        match self {
            ServiceCommand::StartInstance
            | ServiceCommand::StopInstance
            | ServiceCommand::RebootInstance
            | ServiceCommand::TerminateInstance
            | ServiceCommand::DescribeInstance
            | ServiceCommand::DeleteBucket
            | ServiceCommand::ListObjects
            | ServiceCommand::UploadObject
            | ServiceCommand::GetBucketInfo
            | ServiceCommand::StartDatabase
            | ServiceCommand::StopDatabase
            | ServiceCommand::RebootDatabase
            | ServiceCommand::CreateSnapshot
            | ServiceCommand::DescribeDatabase
            | ServiceCommand::DeleteUser
            | ServiceCommand::DeleteRole
            | ServiceCommand::AttachPolicy
            | ServiceCommand::DetachPolicy
            | ServiceCommand::UpdateSecret
            | ServiceCommand::DeleteSecret
            | ServiceCommand::GetSecretValue
            | ServiceCommand::DescribeSecret
            | ServiceCommand::DescribeCluster
            | ServiceCommand::UpdateKubeconfig
            | ServiceCommand::ListNodeGroups
            | ServiceCommand::DeleteCluster => true,

            ServiceCommand::CreateInstance
            | ServiceCommand::ListInstances
            | ServiceCommand::CreateBucket
            | ServiceCommand::DownloadObject
            | ServiceCommand::ListBuckets
            | ServiceCommand::RestoreSnapshot
            | ServiceCommand::ListDatabases
            | ServiceCommand::CreateUser
            | ServiceCommand::CreateRole
            | ServiceCommand::ListUsers
            | ServiceCommand::ListRoles
            | ServiceCommand::CreateSecret
            | ServiceCommand::ListSecrets
            | ServiceCommand::ListClusters
            | ServiceCommand::CreateCluster => false,
        }
    }

    /// Get all service commands for a given service type
    pub fn for_service(service_type: ServiceType) -> Vec<ServiceCommand> {
        match service_type {
            ServiceType::EC2 => vec![
                ServiceCommand::ListInstances,
                ServiceCommand::CreateInstance,
                ServiceCommand::StartInstance,
                ServiceCommand::StopInstance,
                ServiceCommand::RebootInstance,
                ServiceCommand::TerminateInstance,
                ServiceCommand::DescribeInstance,
            ],
            ServiceType::S3 => vec![
                ServiceCommand::ListBuckets,
                ServiceCommand::CreateBucket,
                ServiceCommand::DeleteBucket,
                ServiceCommand::GetBucketInfo,
                ServiceCommand::ListObjects,
                ServiceCommand::UploadObject,
                ServiceCommand::DownloadObject,
            ],
            ServiceType::RDS => vec![
                ServiceCommand::ListDatabases,
                ServiceCommand::StartDatabase,
                ServiceCommand::StopDatabase,
                ServiceCommand::RebootDatabase,
                ServiceCommand::DescribeDatabase,
                ServiceCommand::CreateSnapshot,
                ServiceCommand::RestoreSnapshot,
            ],
            ServiceType::IAM => vec![
                ServiceCommand::ListUsers,
                ServiceCommand::ListRoles,
                ServiceCommand::CreateUser,
                ServiceCommand::DeleteUser,
                ServiceCommand::CreateRole,
                ServiceCommand::DeleteRole,
                ServiceCommand::AttachPolicy,
                ServiceCommand::DetachPolicy,
            ],
            ServiceType::Secrets => vec![
                ServiceCommand::ListSecrets,
                ServiceCommand::CreateSecret,
                ServiceCommand::UpdateSecret,
                ServiceCommand::DeleteSecret,
                ServiceCommand::DescribeSecret,
                ServiceCommand::GetSecretValue,
            ],
            ServiceType::EKS => vec![
                ServiceCommand::ListClusters,
                ServiceCommand::CreateCluster,
                ServiceCommand::DeleteCluster,
                ServiceCommand::DescribeCluster,
                ServiceCommand::UpdateKubeconfig,
                ServiceCommand::ListNodeGroups,
            ],
        }
    }
}

/// Context requirements that determine when a command is available
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextRequirement {
    /// Requires a specific service to be selected
    ServiceSelected(ServiceType),
    /// Requires any resource to be selected
    ResourceSelected,
    /// Requires a specific resource type to be selected
    ResourceOfTypeSelected(ServiceType),
    /// Requires AWS profiles to be available
    ProfilesAvailable,
    /// Requires AWS regions to be available
    RegionsAvailable,
    /// Requires being on a specific page
    OnPage(AppPage),
    /// Requires not being on a specific page
    NotOnPage(AppPage),
}
