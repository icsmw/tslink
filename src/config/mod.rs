use crate::{error::E, package::value, CONFIG};
use cfg::{Cfg, SnakeCaseNaming};
use convert_case::{Case, Casing};
use std::{collections::HashSet, default::Default, fs, path::PathBuf};
use toml::Table;
use uuid::Uuid;

pub mod cfg;

#[derive(Debug, Clone, Default)]
pub struct Config {
    inited: bool,
    cargo: Option<Table>,
    pub node_mod_filename: Option<String>,
    pub node_mod_dist: Option<PathBuf>,
    pub snake_case_naming: HashSet<SnakeCaseNaming>,
    pub exception_suppression: bool,
}

impl Config {
    pub fn overwrite(&mut self, cfg: Option<Cfg>, cargo: Table) -> Result<(), E> {
        self.inited = true;
        self.cargo = Some(cargo);
        if let Some(cfg) = cfg {
            self.node_mod_dist = cfg
                .node
                .clone()
                .map(|s| PathBuf::from(s).parent().map(|p| p.to_path_buf()))
                .unwrap_or(None);
            self.node_mod_filename = cfg
                .node
                .map(|s| {
                    PathBuf::from(s)
                        .file_name()
                        .map(|f| f.to_string_lossy().to_string())
                })
                .unwrap_or(None);
            if let Some(snake_case_naming) = cfg.snake_case_naming {
                snake_case_naming.split(',').for_each(|v| {
                    let condition = SnakeCaseNaming::from_str(v).unwrap();
                    if self.snake_case_naming.get(&condition).is_none() {
                        self.snake_case_naming.insert(condition);
                    }
                });
            }
            self.exception_suppression = if let Some(v) = cfg.exception_suppression {
                v
            } else {
                false
            };
        } else {
            self.node_mod_dist = if self.is_self_testing()? {
                Some(PathBuf::from(format!(
                    "./target/selftests/{}",
                    Uuid::new_v4()
                )))
            } else {
                Some(PathBuf::from("./dist"))
            };
            self.node_mod_filename = Some("index.node".to_string());
        }
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
    let root = std::env::current_dir()?;
    let cargo = root.join("Cargo.toml");
    let config = root.join("tslink.toml");
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
            if config.exists() {
                Some(toml::from_str(&fs::read_to_string(config)?)?)
            } else {
                None
            },
            toml::from_str(&fs::read_to_string(cargo)?)?,
        )?;
    Ok(())
}

pub fn get() -> Result<Config, E> {
    Ok(CONFIG
        .read()
        .map_err(|e| E::AccessError(e.to_string()))?
        .clone())
}
