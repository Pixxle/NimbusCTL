use thiserror::Error;

#[derive(Error, Debug)]
pub enum AppError {
    #[error("AWS SDK error: {0}")]
    AwsSdk(String),

    #[error("AWS configuration error: {0}")]
    AwsConfig(String),

    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),

    #[error("Serialization error: {0}")]
    Serde(#[from] serde_json::Error),

    #[error("Configuration error: {0}")]
    Config(String),

    #[error("Profile error: {0}")]
    Profile(String),

    #[error("Resource not found: {0}")]
    ResourceNotFound(String),

    #[error("Authentication error: {0}")]
    Auth(String),

    #[error("Network error: {0}")]
    Network(String),

    #[error("Parsing error: {0}")]
    Parse(String),

    #[error("General error: {0}")]
    General(String),
}

impl From<Box<dyn std::error::Error + Send + Sync>> for AppError {
    fn from(err: Box<dyn std::error::Error + Send + Sync>) -> Self {
        AppError::General(err.to_string())
    }
}

impl From<Box<dyn std::error::Error>> for AppError {
    fn from(err: Box<dyn std::error::Error>) -> Self {
        AppError::General(err.to_string())
    }
}

impl From<&str> for AppError {
    fn from(err: &str) -> Self {
        AppError::General(err.to_string())
    }
}

impl From<String> for AppError {
    fn from(err: String) -> Self {
        AppError::General(err)
    }
}

pub type Result<T> = std::result::Result<T, AppError>;
