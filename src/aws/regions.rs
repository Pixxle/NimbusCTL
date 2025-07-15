use crate::aws::types::AwsRegion;

pub struct RegionManager;

impl RegionManager {
    pub fn get_all_regions() -> Vec<AwsRegion> {
        vec![
            AwsRegion {
                name: "us-east-1".to_string(),
                display_name: "US East (N. Virginia)".to_string(),
            },
            AwsRegion {
                name: "us-east-2".to_string(),
                display_name: "US East (Ohio)".to_string(),
            },
            AwsRegion {
                name: "us-west-1".to_string(),
                display_name: "US West (N. California)".to_string(),
            },
            AwsRegion {
                name: "us-west-2".to_string(),
                display_name: "US West (Oregon)".to_string(),
            },
            AwsRegion {
                name: "eu-west-1".to_string(),
                display_name: "Europe (Ireland)".to_string(),
            },
            AwsRegion {
                name: "eu-west-2".to_string(),
                display_name: "Europe (London)".to_string(),
            },
            AwsRegion {
                name: "eu-west-3".to_string(),
                display_name: "Europe (Paris)".to_string(),
            },
            AwsRegion {
                name: "eu-central-1".to_string(),
                display_name: "Europe (Frankfurt)".to_string(),
            },
            AwsRegion {
                name: "ap-northeast-1".to_string(),
                display_name: "Asia Pacific (Tokyo)".to_string(),
            },
            AwsRegion {
                name: "ap-northeast-2".to_string(),
                display_name: "Asia Pacific (Seoul)".to_string(),
            },
            AwsRegion {
                name: "ap-southeast-1".to_string(),
                display_name: "Asia Pacific (Singapore)".to_string(),
            },
            AwsRegion {
                name: "ap-southeast-2".to_string(),
                display_name: "Asia Pacific (Sydney)".to_string(),
            },
            AwsRegion {
                name: "ap-south-1".to_string(),
                display_name: "Asia Pacific (Mumbai)".to_string(),
            },
            AwsRegion {
                name: "sa-east-1".to_string(),
                display_name: "South America (São Paulo)".to_string(),
            },
        ]
    }

    pub fn get_region_by_name(name: &str) -> Option<AwsRegion> {
        Self::get_all_regions().into_iter().find(|r| r.name == name)
    }

    pub fn is_valid_region(name: &str) -> bool {
        Self::get_all_regions().iter().any(|r| r.name == name)
    }
}
