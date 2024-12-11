use serde::Deserialize;
use std::{
    collections::HashMap,
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

#[derive(Deserialize, Debug, Clone, Default)]
pub enum EnumRepresentation {
    Collapsed,
    #[default]
    AsInterface,
    AsType,
}

impl TryFrom<&str> for EnumRepresentation {
    type Error = Error;
    fn try_from(value: &str) -> Result<EnumRepresentation, Self::Error> {
        if value == EnumRepresentation::Collapsed.to_string() {
            Ok(EnumRepresentation::Collapsed)
        } else if value == EnumRepresentation::AsInterface.to_string() {
            Ok(EnumRepresentation::AsInterface)
        } else if value == EnumRepresentation::AsType.to_string() {
            Ok(EnumRepresentation::AsType)
        } else {
            Err(Error::new(
                ErrorKind::Other,
                format!("Unknown option for enum_representation option: \"{value}\""),
            ))
        }
    }
}

impl fmt::Display for EnumRepresentation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::Collapsed => "collapsed",
                Self::AsInterface => "as_interface",
                Self::AsType => "as_type",
            }
        )
    }
}

impl TryFrom<&str> for SnakeCaseNaming {
    type Error = Error;
    fn try_from(value: &str) -> Result<SnakeCaseNaming, Self::Error> {
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
    pub type_map: HashMap<String, String>,
    pub enum_representation: EnumRepresentation,
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
                type_map: settings
                    .get("type_map")
                    .and_then(|v| v.as_table())
                    .map(|m| {
                        m.iter()
                            .filter_map(|(key, value)| {
                                value.as_str().map(|v| (key.clone(), v.to_string()))
                            })
                            .collect()
                    })
                    .unwrap_or_default(),
                enum_representation: settings
                    .get("enum_representation")
                    .and_then(|v| v.as_str().map(|s| s.try_into().unwrap_or_default()))
                    .unwrap_or_default(),
            })
            .unwrap_or_default()
    }
}
