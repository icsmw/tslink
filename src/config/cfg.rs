use serde::Deserialize;
use std::{
    fmt,
    io::{Error, ErrorKind},
};
use toml::Table;

const TSLINK_CARGO_KEY: &str = "tslink";

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

#[derive(Deserialize, Debug, Default)]
pub struct Cfg {
    pub node: Option<String>,
    pub snake_case_naming: Option<String>,
    pub exception_suppression: bool,
    pub int_over_32_as_big_int: bool,
}

impl Cfg {
    pub fn new(cargo: &Table) -> Self {
        cargo
            .get(TSLINK_CARGO_KEY)
            .and_then(|v| v.as_table())
            .map(|settings| Cfg {
                node: settings
                    .get("node")
                    .and_then(|v| v.as_str().map(|v| v.to_string())),
                snake_case_naming: settings
                    .get("snake_case_naming")
                    .and_then(|v| v.as_str().map(|v| v.to_string())),
                exception_suppression: settings
                    .get("exception_suppression")
                    .and_then(|v| v.as_bool())
                    .unwrap_or_default(),
                int_over_32_as_big_int: settings
                    .get("int_over_32_as_big_int")
                    .and_then(|v| v.as_bool())
                    .unwrap_or_default(),
            })
            .unwrap_or_default()
    }
}
