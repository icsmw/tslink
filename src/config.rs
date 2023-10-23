use crate::{args::Args, CONFIG};
use serde::Deserialize;
use std::{
    default::Default,
    env::current_dir,
    fs,
    io::{Error, ErrorKind},
    path::PathBuf,
};

#[derive(Deserialize, Debug)]
pub struct Cfg {
    pub path: Option<String>,
}

#[derive(Deserialize, Debug)]
pub struct Config {
    #[serde(skip)]
    inited: bool,
    #[serde(skip)]
    path_buf: Option<PathBuf>,
    pub path: Option<String>,
}

impl Default for Config {
    fn default() -> Self {
        Config {
            path: None,
            path_buf: None,
            inited: false,
        }
    }
}

impl Config {
    pub fn overwrite(&mut self, cfg: Cfg) {
        self.inited = true;
        self.path = cfg.path.clone();
        self.path_buf = cfg.path.map(|s| PathBuf::from(s));
        println!("{self:?}");
    }

    pub fn get_path(&self, args: Option<&Args>, ext: &str) -> Result<PathBuf, Error> {
        let path = if let Some(args) = args {
            args.path()
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
}

pub fn setup() -> Result<(), String> {
    if CONFIG.read().map_err(|e| e.to_string())?.inited {
        return Ok(());
    }
    let config = std::env::current_dir().map_or(None, |p| {
        let config = p.join("tslink.toml");
        if config.exists() {
            Some(config)
        } else {
            None
        }
    });
    if let Some(config) = config {
        let cfg: Cfg = toml::from_str(&fs::read_to_string(config).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
        CONFIG.write().map_err(|e| e.to_string())?.overwrite(cfg);
    }
    Ok(())
}
