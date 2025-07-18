use crate::app::state::AppPage;
use crate::aws::types::{AwsProfile, AwsRegion, ResourceId, ServiceType};
use crate::command::commands::ContextRequirement;

/// Context information used to determine which commands are available
#[derive(Debug, Clone)]
pub struct CommandContext {
    /// Current page the user is on
    pub current_page: AppPage,
    /// Currently selected service (if any)
    pub selected_service: Option<ServiceType>,
    /// Currently selected resource (if any)
    pub selected_resource: Option<ResourceId>,
    /// Available AWS profiles
    pub available_profiles: Vec<AwsProfile>,
    /// Available AWS regions
    pub available_regions: Vec<AwsRegion>,
    /// Current AWS profile
    pub current_profile: String,
    /// Current AWS region
    pub current_region: String,
}

impl CommandContext {
    /// Create a new command context
    pub fn new(
        current_page: AppPage,
        selected_service: Option<ServiceType>,
        selected_resource: Option<ResourceId>,
        available_profiles: Vec<AwsProfile>,
        available_regions: Vec<AwsRegion>,
        current_profile: String,
        current_region: String,
    ) -> Self {
        Self {
            current_page,
            selected_service,
            selected_resource,
            available_profiles,
            available_regions,
            current_profile,
            current_region,
        }
    }

    /// Check if a context requirement is satisfied
    pub fn satisfies_requirement(&self, requirement: &ContextRequirement) -> bool {
        match requirement {
            ContextRequirement::ServiceSelected(service_type) => {
                self.selected_service == Some(*service_type)
            }
            ContextRequirement::ResourceSelected => self.selected_resource.is_some(),
            ContextRequirement::ResourceOfTypeSelected(service_type) => {
                self.selected_resource.is_some() && self.selected_service == Some(*service_type)
            }
            ContextRequirement::ProfilesAvailable => !self.available_profiles.is_empty(),
            ContextRequirement::RegionsAvailable => !self.available_regions.is_empty(),
            ContextRequirement::OnPage(page) => self.current_page == *page,
            ContextRequirement::NotOnPage(page) => self.current_page != *page,
        }
    }

    /// Check if all requirements in a list are satisfied
    pub fn satisfies_all_requirements(&self, requirements: &[ContextRequirement]) -> bool {
        requirements
            .iter()
            .all(|req| self.satisfies_requirement(req))
    }

    /// Get the service type from the current page if applicable
    pub fn get_service_from_page(&self) -> Option<ServiceType> {
        match &self.current_page {
            AppPage::ResourceList(service_type) => Some(*service_type),
            AppPage::ResourceDetail(service_type, _) => Some(*service_type),
            _ => None,
        }
    }

    /// Check if currently viewing a specific service
    pub fn is_viewing_service(&self, service_type: ServiceType) -> bool {
        self.get_service_from_page() == Some(service_type)
    }

    /// Check if currently on dashboard
    pub fn is_on_dashboard(&self) -> bool {
        matches!(self.current_page, AppPage::Dashboard)
    }

    /// Check if currently viewing a resource list
    pub fn is_viewing_resource_list(&self) -> bool {
        matches!(self.current_page, AppPage::ResourceList(_))
    }

    /// Check if currently viewing resource details
    pub fn is_viewing_resource_detail(&self) -> bool {
        matches!(self.current_page, AppPage::ResourceDetail(_, _))
    }

    /// Check if currently on settings page
    pub fn is_on_settings(&self) -> bool {
        matches!(self.current_page, AppPage::Settings)
    }

    /// Get available profile names
    pub fn get_profile_names(&self) -> Vec<String> {
        self.available_profiles
            .iter()
            .map(|p| p.name.clone())
            .collect()
    }

    /// Get available region names
    pub fn get_region_names(&self) -> Vec<String> {
        self.available_regions
            .iter()
            .map(|r| r.name.clone())
            .collect()
    }

    /// Check if a profile is available
    pub fn has_profile(&self, profile_name: &str) -> bool {
        self.available_profiles
            .iter()
            .any(|p| p.name == profile_name)
    }

    /// Check if a region is available
    pub fn has_region(&self, region_name: &str) -> bool {
        self.available_regions.iter().any(|r| r.name == region_name)
    }

    /// Update the context with new page information
    pub fn with_page(mut self, page: AppPage) -> Self {
        self.current_page = page;
        // Update selected service based on page
        self.selected_service = self.get_service_from_page();
        self
    }

    /// Update the context with new selected resource
    pub fn with_selected_resource(mut self, resource_id: Option<ResourceId>) -> Self {
        self.selected_resource = resource_id;
        self
    }

    /// Update the context with new selected service
    pub fn with_selected_service(mut self, service_type: Option<ServiceType>) -> Self {
        self.selected_service = service_type;
        self
    }
}
