use crate::config::Config;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("Access error: {0}")]
    AccessError(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("Toml error")]
    PasringToml(#[from] toml::de::Error),
    #[error("Configuration error")]
    Configuration(#[from] std::io::Error),
    #[error("Not supported. Try to ignore it with #[tslink(ignore)]")]
    NotSupported,
    #[error("Fail to identify name of entity")]
    FailIdentify,
    #[error("Fail to find parent struct. Make sure attribute #[tslink] had been added on struct declaration")]
    NotFoundStruct,
    #[error("Fail to parse context: {0}")]
    PasringContext(String),
    #[error("Parsing error: {0}")]
    Parsing(String),
    #[error("Entity already has been read: {0}")]
    EntityExist(String),
    #[error("File not found: {0}")]
    FileNotFound(String),
    #[error("{0}")]
    Other(String),
}
