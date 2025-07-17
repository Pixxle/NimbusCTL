#[cfg(test)]
mod tests {
    use super::*;
    use crate::app::state::AppPage;
    use crate::aws::types::{AwsProfile, AwsRegion, ServiceType};
    use crate::command::{CommandContext, CommandRegistry};

    fn create_test_context() -> CommandContext {
        let profiles = vec![
            AwsProfile {
                name: "default".to_string(),
                region: Some("us-east-1".to_string()),
                access_key_id: None,
                secret_access_key: None,
                session_token: None,
                role_arn: None,
                source_profile: None,
            },
            AwsProfile {
                name: "dev".to_string(),
                region: Some("us-west-2".to_string()),
                access_key_id: None,
                secret_access_key: None,
                session_token: None,
                role_arn: None,
                source_profile: None,
            },
        ];

        let regions = vec![
            AwsRegion {
                name: "us-east-1".to_string(),
                display_name: "US East (N. Virginia)".to_string(),
            },
            AwsRegion {
                name: "us-west-2".to_string(),
                display_name: "US West (Oregon)".to_string(),
            },
        ];

        CommandContext::new(
            AppPage::Dashboard,
            None,
            None,
            profiles,
            regions,
            "default".to_string(),
            "us-east-1".to_string(),
        )
    }

    #[test]
    fn test_context_aware_command_generation() {
        let context = create_test_context();
        let commands = CommandRegistry::get_context_aware_commands(&context);

        // Should have navigation, profile, region, and general commands
        assert!(!commands.is_empty());

        // Should have profile switching commands for non-current profiles
        let profile_commands: Vec<_> = commands
            .iter()
            .filter(|cmd| matches!(cmd.category, crate::command::CommandCategory::Profile))
            .collect();
        assert!(!profile_commands.is_empty());

        // Should have region switching commands for non-current regions
        let region_commands: Vec<_> = commands
            .iter()
            .filter(|cmd| matches!(cmd.category, crate::command::CommandCategory::Region))
            .collect();
        assert!(!region_commands.is_empty());
    }

    #[test]
    fn test_service_specific_commands_when_service_selected() {
        let mut context = create_test_context();
        context.current_page = AppPage::ResourceList(ServiceType::EC2);
        context.selected_service = Some(ServiceType::EC2);

        let commands = CommandRegistry::get_context_aware_commands(&context);

        // Should have EC2-specific commands
        let ec2_commands: Vec<_> = commands
            .iter()
            .filter(|cmd| {
                matches!(
                    cmd.category,
                    crate::command::CommandCategory::Service(ServiceType::EC2)
                )
            })
            .collect();
        assert!(!ec2_commands.is_empty());

        // List commands should be enabled (no resource selection required)
        let list_command = ec2_commands
            .iter()
            .find(|cmd| cmd.name.contains("List EC2 Instances"));
        assert!(list_command.is_some());
        assert!(list_command.unwrap().enabled);
    }

    #[test]
    fn test_resource_specific_commands_when_resource_selected() {
        let mut context = create_test_context();
        context.current_page =
            AppPage::ResourceDetail(ServiceType::EC2, "i-1234567890abcdef0".to_string());
        context.selected_service = Some(ServiceType::EC2);
        context.selected_resource = Some("i-1234567890abcdef0".to_string());

        let commands = CommandRegistry::get_context_aware_commands(&context);

        // Should have EC2-specific commands
        let ec2_commands: Vec<_> = commands
            .iter()
            .filter(|cmd| {
                matches!(
                    cmd.category,
                    crate::command::CommandCategory::Service(ServiceType::EC2)
                )
            })
            .collect();
        assert!(!ec2_commands.is_empty());

        // Resource-specific commands should be enabled
        let start_command = ec2_commands
            .iter()
            .find(|cmd| cmd.name.contains("Start Instance"));
        assert!(start_command.is_some());
        assert!(start_command.unwrap().enabled);
    }

    #[test]
    fn test_context_requirement_checking() {
        let context = create_test_context();

        // Test ProfilesAvailable requirement
        assert!(
            context.satisfies_requirement(&crate::command::ContextRequirement::ProfilesAvailable)
        );

        // Test RegionsAvailable requirement
        assert!(
            context.satisfies_requirement(&crate::command::ContextRequirement::RegionsAvailable)
        );

        // Test ServiceSelected requirement (should fail when no service selected)
        assert!(!context.satisfies_requirement(
            &crate::command::ContextRequirement::ServiceSelected(ServiceType::EC2)
        ));

        // Test NotOnPage requirement
        assert!(
            context.satisfies_requirement(&crate::command::ContextRequirement::NotOnPage(
                AppPage::Settings
            ))
        );
        assert!(
            !context.satisfies_requirement(&crate::command::ContextRequirement::NotOnPage(
                AppPage::Dashboard
            ))
        );
    }

    #[test]
    fn test_command_filtering_by_context() {
        let mut context = create_test_context();

        // Test dashboard context - should not have service-specific commands
        let dashboard_commands = CommandRegistry::get_context_aware_commands(&context);
        let service_commands: Vec<_> = dashboard_commands
            .iter()
            .filter(|cmd| matches!(cmd.category, crate::command::CommandCategory::Service(_)))
            .collect();
        // Should have minimal or no service commands when on dashboard
        assert!(service_commands.len() < 10); // Arbitrary threshold

        // Test service context - should have service-specific commands
        context.current_page = AppPage::ResourceList(ServiceType::S3);
        context.selected_service = Some(ServiceType::S3);
        let service_commands = CommandRegistry::get_context_aware_commands(&context);
        let s3_commands: Vec<_> = service_commands
            .iter()
            .filter(|cmd| {
                matches!(
                    cmd.category,
                    crate::command::CommandCategory::Service(ServiceType::S3)
                )
            })
            .collect();
        assert!(!s3_commands.is_empty());
    }
}
