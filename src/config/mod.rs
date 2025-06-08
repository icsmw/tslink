use crate::{error::E, package::value, CONFIG};
use cfg::{Cfg, EnumRepresentation, SnakeCaseNaming};
use convert_case::{Case, Casing};
use std::{
    collections::{HashMap, HashSet},
    default::Default,
    env, fs,
    path::PathBuf,
};
use toml::Table;
use uuid::Uuid;

pub mod cfg;

const TSLINK_BUILD_ENV: &str = "TSLINK_BUILD";

/// Global configuration settings for the code generation process.
///
/// `Config` aggregates environment-specific, Cargo-derived, and user-defined settings
/// that influence how Rust types and functions are translated into JavaScript/TypeScript code.
///
/// It is typically constructed once (e.g., from `Cargo.toml` or a macro environment)
/// and then queried during macro execution or file generation.
#[derive(Debug, Clone, Default)]
pub struct Config {
    /// Indicates whether the configuration has been initialized.
    ///
    /// Used to prevent re-initialization or partial access before setup.
    inited: bool,

    /// Raw parsed contents of `[package.metadata.tslink]` or `[tslink]` from `Cargo.toml`, if available.
    ///
    /// May be used to retrieve user-defined keys not explicitly mapped to struct fields.
    cargo: Option<Table>,

    /// Controls whether file system writes (e.g., `.ts`, `.d.ts`) are allowed.
    ///
    /// Typically disabled during procedural macro expansion or in sandboxed environments.
    pub io_allowed: bool,

    /// Optional filename for the generated Node.js module (e.g., `"lib.node"`).
    ///
    /// Used when producing JavaScript FFI wrappers or distributing compiled bindings.
    pub node_mod_filename: Option<String>,

    /// Optional path to the output directory for the Node.js module or compiled artifacts.
    pub node_mod_dist: Option<PathBuf>,

    /// Set of entities (fields, methods, etc.) that should be renamed from `snake_case` to `camelCase` in TypeScript.
    ///
    /// Useful for maintaining idiomatic JS/TS naming conventions while preserving Rust semantics.
    pub snake_case_naming: HashSet<SnakeCaseNaming>,

    /// Enables exception suppression in generated JS/TS code.
    ///
    /// When active, JS wrappers will wrap function calls in `try/catch` blocks,
    /// and TypeScript return types will be `T | Error`.
    pub exception_suppression: bool,

    /// Treats all integer types with width > 32 bits (e.g., `i64`, `u64`) as `bigint` in TypeScript.
    ///
    /// Prevents precision loss and improves correctness across language boundaries.
    pub int_over_32_as_big_int: bool,

    /// Manual mapping of Rust type names (as strings) to target TypeScript types.
    ///
    /// This allows overriding or customizing how specific Rust types appear in `.ts` or `.d.ts` files.
    pub type_map: HashMap<String, String>,

    /// Determines how enums are rendered in TypeScript (e.g., flat union, discriminated union).
    ///
    /// Controlled via `[tslink.enum_representation]` or macro-level overrides.
    pub enum_representation: EnumRepresentation,
}

impl Config {
    pub fn overwrite(&mut self, cargo: Table, io_allowed: bool) -> Result<(), E> {
        self.inited = true;
        self.io_allowed = io_allowed;
        let cfg = Cfg::new(&cargo);
        self.cargo = Some(cargo);
        let is_self_testing = self.is_self_testing()?;
        self.node_mod_dist = cfg
            .node
            .clone()
            .and_then(|s| PathBuf::from(s).parent().map(|p| p.to_path_buf()))
            .or_else(|| {
                Some(if is_self_testing {
                    PathBuf::from(format!("./target/selftests/{}", Uuid::new_v4()))
                } else {
                    PathBuf::from("./dist")
                })
            });
        self.node_mod_filename = cfg.node.and_then(|s| {
            PathBuf::from(s)
                .file_name()
                .map(|f| f.to_string_lossy().to_string())
        });
        if let Some(snake_case_naming) = cfg.snake_case_naming {
            for case in snake_case_naming.split(',') {
                let condition: SnakeCaseNaming = case.try_into()?;
                if !self.snake_case_naming.contains(&condition) {
                    self.snake_case_naming.insert(condition);
                }
            }
        }
        self.exception_suppression = cfg.exception_suppression;
        self.int_over_32_as_big_int = cfg.int_over_32_as_big_int;
        self.type_map = cfg.type_map;
        self.enum_representation = cfg.enum_representation;
        Ok(())
    }

    pub fn get_cargo(&self) -> &Table {
        self.cargo.as_ref().expect("Cargo.toml should be available")
    }

    pub fn is_self_testing(&self) -> Result<bool, E> {
        let package = self
            .get_cargo()
            .get("package")
            .ok_or(E::Other(String::from(
                "Fail to find [package] in Cargo.toml",
            )))?;
        Ok(value(package, "name")? == *"tslink")
    }

    pub fn rename_field(&self, origin: &str) -> String {
        if self.snake_case_naming.contains(&SnakeCaseNaming::Fields) {
            origin.to_case(Case::Camel)
        } else {
            origin.to_owned()
        }
    }

    pub fn rename_method(&self, origin: &str) -> String {
        if self.snake_case_naming.contains(&SnakeCaseNaming::Methods) {
            origin.to_case(Case::Camel)
        } else {
            origin.to_owned()
        }
    }

    pub fn overwrite_reftype<S: AsRef<str>>(&self, origin: S) -> String {
        self.type_map
            .get(origin.as_ref())
            .map(|s| s.to_owned())
            .unwrap_or(origin.as_ref().to_owned())
    }
}

pub fn setup() -> Result<(), E> {
    if CONFIG
        .read()
        .map_err(|e| E::AccessError(e.to_string()))?
        .inited
    {
        return Ok(());
    }
    let root = match env::var("CARGO_MANIFEST_DIR") {
        Ok(manifest_dir) => PathBuf::from(manifest_dir),
        Err(_) => std::env::current_dir()?,
    };
    let cargo = root.join("Cargo.toml");
    if !cargo.exists() {
        return Err(E::FileNotFound(format!(
            "Cargo.toml isn't found in {}",
            root.to_string_lossy()
        )));
    }
    CONFIG
        .write()
        .map_err(|e| E::AccessError(e.to_string()))?
        .overwrite(
            toml::from_str(&fs::read_to_string(cargo)?)?,
            env::var_os(TSLINK_BUILD_ENV).is_some_and(|v| {
                ["1", "true", "on"].contains(&v.to_string_lossy().to_lowercase().trim())
            }),
        )?;
    Ok(())
}

pub fn get() -> Result<Config, E> {
    Ok(CONFIG
        .read()
        .map_err(|e| E::AccessError(e.to_string()))?
        .clone())
}
