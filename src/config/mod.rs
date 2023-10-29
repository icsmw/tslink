use crate::{context::Context, error::E, CONFIG};
use cfg::{Cfg, SnakeCaseNaming};
use convert_case::{Case, Casing};
use std::{
    collections::HashSet,
    default::Default,
    env::current_dir,
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};
use toml::Table;

pub mod cfg;

#[derive(Debug, Clone)]
pub struct Config {
    inited: bool,
    path_buf: Option<PathBuf>,
    cargo: Option<Table>,
    pub node_mod_filename: Option<String>,
    pub node_mod_dist: Option<PathBuf>,
    pub snake_case_naming: HashSet<SnakeCaseNaming>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path_buf: None,
            node_mod_filename: None,
            node_mod_dist: None,
            snake_case_naming: HashSet::new(),
            cargo: None,
            inited: false,
        }
    }
}

impl Config {
    pub fn overwrite(&mut self, cfg: Cfg, cargo: Table) {
        self.inited = true;
        self.path_buf = cfg.path.map(|s| PathBuf::from(s));
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
        self.cargo = Some(cargo);
    }

    pub fn get_path(&self, context: Option<&Context>, ext: &str) -> Result<PathBuf, E> {
        let path = if let Some(context) = context {
            context.path()
        } else {
            None
        };
        let path = path
            .or_else(|| self.path_buf.clone())
            .ok_or(Error::new(ErrorKind::Other, "Dest path isn't set"))?;
        let mut path = if path.is_relative() {
            current_dir()?.join(path)
        } else {
            path
        };
        if let Some(ex) = path.extension() {
            if let Some(ex) = ex.to_str() {
                if ex != ext {
                    path.set_extension(ext);
                }
            }
        } else {
            path.set_extension(ext);
        }
        Ok(path)
    }

    pub fn get_cargo(&self) -> &Table {
        &self.cargo.as_ref().expect("Cargo.toml should be available")
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
    if !config.exists() {
        return Err(E::FileNotFound(format!(
            "tslink.toml isn't found in {}",
            root.to_string_lossy()
        )));
    }
    CONFIG
        .write()
        .map_err(|e| E::AccessError(e.to_string()))?
        .overwrite(
            toml::from_str(&fs::read_to_string(config)?)?,
            toml::from_str(&fs::read_to_string(cargo)?)?,
        );
    Ok(())
}

pub fn get() -> Result<Config, E> {
    Ok(CONFIG
        .read()
        .map_err(|e| E::AccessError(e.to_string()))?
        .clone())
}
