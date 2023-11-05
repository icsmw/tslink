use serde::Deserialize;
use std::{
    fmt,
    io::{Error, ErrorKind},
};

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SnakeCaseNaming {
    Methods,
    Fields,
}

impl SnakeCaseNaming {
    pub fn from_str(value: &str) -> Result<Self, Error> {
        if value == SnakeCaseNaming::Methods.to_string() {
            Ok(SnakeCaseNaming::Methods)
        } else if value == SnakeCaseNaming::Fields.to_string() {
            Ok(SnakeCaseNaming::Fields)
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!("Unknown option for snake_case_naming option: \"{value}\""),
            ))
        }
    }
}

impl fmt::Display for SnakeCaseNaming {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Methods => "methods",
                Self::Fields => "fields",
            }
        )
    }
}

#[derive(Deserialize, Debug)]
pub struct Cfg {
    pub path: Option<String>,
    pub node: Option<String>,
    pub snake_case_naming: Option<String>,
    pub exception_suppression: Option<bool>,
}
