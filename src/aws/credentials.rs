use crate::utils::error::Result;

pub struct CredentialsValidator {
    // STS client will be added back in Phase 2
}

impl CredentialsValidator {
    pub fn new() -> Self {
        Self {}
    }
    
    pub async fn validate_credentials(&self) -> Result<CredentialsInfo> {
        // For Phase 1, return mock credentials info
        Ok(CredentialsInfo {
            user_id: "mock-user".to_string(),
            account: "123456789012".to_string(),
            arn: "arn:aws:iam::123456789012:user/mock-user".to_string(),
            valid: true,
        })
    }
}

#[derive(Debug, Clone)]
pub struct CredentialsInfo {
    pub user_id: String,
    pub account: String,
    pub arn: String,
    pub valid: bool,
}