use crate::{error::E, package::value, CONFIG};
use cfg::{Cfg, SnakeCaseNaming};
use convert_case::{Case, Casing};
use std::{collections::HashSet, default::Default, env, fs, path::PathBuf};
use toml::Table;
use uuid::Uuid;

pub mod cfg;

const TSLINK_BUILD_ENV: &str = "TSLINK_BUILD";

#[derive(Debug, Clone, Default)]
pub struct Config {
    inited: bool,
    cargo: Option<Table>,
    pub io_allowed: bool,
    pub node_mod_filename: Option<String>,
    pub node_mod_dist: Option<PathBuf>,
    pub snake_case_naming: HashSet<SnakeCaseNaming>,
    pub exception_suppression: bool,
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
        self.node_mod_filename = cfg
            .node
            .and_then(|s| {
                PathBuf::from(s)
                    .file_name()
                    .map(|f| f.to_string_lossy().to_string())
            })
            .or_else(|| Some("index.node".to_string()));
        if let Some(snake_case_naming) = cfg.snake_case_naming {
            snake_case_naming.split(',').for_each(|v| {
                let condition = SnakeCaseNaming::from_str(v).unwrap();
                if self.snake_case_naming.get(&condition).is_none() {
                    self.snake_case_naming.insert(condition);
                }
            });
        }
        self.exception_suppression = cfg.exception_suppression.unwrap_or(false);
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
        if self
            .snake_case_naming
            .get(&SnakeCaseNaming::Fields)
            .is_some()
        {
            origin.to_case(Case::Camel)
        } else {
            origin.to_owned()
        }
    }

    pub fn rename_method(&self, origin: &str) -> String {
        if self
            .snake_case_naming
            .get(&SnakeCaseNaming::Methods)
            .is_some()
        {
            origin.to_case(Case::Camel)
        } else {
            origin.to_owned()
        }
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
    let manifest_dir = env::var("CARGO_MANIFEST_DIR").expect("CARGO_MANIFEST_DIR is not set");
    let root = PathBuf::from(manifest_dir);

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
            env::var_os(TSLINK_BUILD_ENV).map_or(false, |v| {
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
