use std::convert::TryInto;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum E {
    #[error("Access error: {0}")]
    AccessError(String),
    #[error("Invalid configuration: {0}")]
    InvalidConfiguration(String),
    #[error("Toml error")]
    PasringToml(#[from] toml::de::Error),
    #[error("IO error")]
    IO(#[from] std::io::Error),
    #[error("LexError error")]
    LexError(#[from] proc_macro2::LexError),
    #[error("Not supported. Try to ignore it with #[tslink(ignore)]: {0}")]
    NotSupported(String),
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
    #[error("Compiler error")]
    Compiler(syn::Error),
    #[error("{0}")]
    Other(String),
}

impl TryInto<syn::Error> for E {
    type Error = String;

    fn try_into(self) -> Result<syn::Error, String> {
        if let Self::Compiler(err) = self {
            Ok(err)
        } else {
            Err(self.to_string())
        }
    }
}
