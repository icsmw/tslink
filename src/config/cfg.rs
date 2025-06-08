use serde::Deserialize;
use std::{collections::HashMap, fmt, io::Error};
use toml::Table;

const TSLINK_CARGO_KEY: &str = "tslink";
const PACKAGE_CARGO_KEY: &str = "package";
const METADATA_CARGO_KEY: &str = "metadata";

#[derive(Clone, Debug, Eq, Hash, PartialEq)]
pub enum SnakeCaseNaming {
    Methods,
    Fields,
}

/// Specifies how a Rust `enum` should be represented in the generated TypeScript definitions.
///
/// This enum controls the output format for enums with mixed variants — including unit, tuple, and struct-like variants —
/// to ensure compatibility and clarity on the TypeScript side.
///
/// The following Rust example:
/// ```ignore
/// enum SomeEnum {
///     One,
///     Two,
///     Three(u8),
///     Four(u8, u16, u32),
///     Five((String, String)),
///     Six(Vec<u8>),
/// }
/// ```
/// can be transformed into TypeScript in different ways depending on the selected representation.
#[derive(Deserialize, Debug, Clone, Default)]
pub enum EnumRepresentation {
    /// Generates a single interface with optional discriminant fields.
    ///
    /// ```ignore
    /// export interface SomeEnum {
    ///     One?: null;
    ///     Two?: null;
    ///     Three?: number;
    ///     Four?: [number, number, number];
    ///     Five?: [string, string];
    ///     Six?: number[];
    /// }
    /// ```
    DiscriminatedUnion,

    /// Generates a flat tagged union using TypeScript's `type` union.
    ///
    /// This is the default.
    ///
    /// ```ignore
    /// export type SomeEnum =
    ///     { One: null; } |
    ///     { Two: null; } |
    ///     { Three: number; } |
    ///     { Four: [number, number, number]; } |
    ///     { Five: [string, string]; } |
    ///     { Six: number[]; };
    /// ```
    #[default]
    Flat,

    /// Generates a mixed union of string literals and tagged object variants.
    ///
    /// Useful when some variants are unit-like and others carry data.
    ///
    /// ```ignore
    /// export type SomeEnum =
    ///     "One" |
    ///     "Two" |
    ///     "Three" |
    ///     { Four: [number, number, number]; } |
    ///     { Five: [string, string]; } |
    ///     { Six: number[]; };
    /// ```
    Union,
}

impl TryFrom<&str> for EnumRepresentation {
    type Error = Error;
    fn try_from(value: &str) -> Result<EnumRepresentation, Self::Error> {
        if value == EnumRepresentation::DiscriminatedUnion.to_string() {
            Ok(EnumRepresentation::DiscriminatedUnion)
        } else if value == EnumRepresentation::Flat.to_string() {
            Ok(EnumRepresentation::Flat)
        } else if value == EnumRepresentation::Union.to_string() {
            Ok(EnumRepresentation::Union)
        } else {
            Err(Error::other(format!(
                "Unknown option for enum_representation option: \"{value}\""
            )))
        }
    }
}

impl fmt::Display for EnumRepresentation {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Self::DiscriminatedUnion => "discriminated",
                Self::Flat => "flat",
                Self::Union => "union",
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
            Err(Error::other(format!(
                "Unknown option for snake_case_naming option: \"{value}\""
            )))
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
        if cargo.contains_key(TSLINK_CARGO_KEY) {
            // Support 0.4.1 > versions
            cargo.get(TSLINK_CARGO_KEY)
        } else {
            // Default settings place after 0.4.1
            cargo
                .get(PACKAGE_CARGO_KEY)
                .and_then(|p| p.get(METADATA_CARGO_KEY))
                .and_then(|m| m.get(TSLINK_CARGO_KEY))
        }
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
