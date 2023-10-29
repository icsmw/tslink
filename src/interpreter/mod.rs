mod dts;
mod js;
mod offset;
mod ts;

use crate::{config, error::E, nature::Natures};
pub use offset::Offset;
use std::{
    collections::HashSet,
    fs::{self, File, OpenOptions},
    path::PathBuf,
};

pub fn create_node_located_file(
    file_name: &str,
    dropped: &mut HashSet<PathBuf>,
) -> Result<File, E> {
    let path = config::get()?
        .node_mod_dist
        .clone()
        .ok_or(E::InvalidConfiguration(String::from(
            "No path to folder with node module. Set correct path in tslink.toml; field \"node\"",
        )))?
        .join(file_name);
    if dropped.contains(&path) {
        return Ok(OpenOptions::new().write(true).append(true).open(&path)?);
    }
    if path.exists() {
        fs::remove_file(&path)?;
        let _ = dropped.insert(path.clone());
    }
    File::create(&path)?;
    Ok(OpenOptions::new().write(true).append(true).open(&path)?)
}

pub fn ts(natures: &Natures) -> Result<(), E> {
    let mut dropped: HashSet<PathBuf> = HashSet::new();
    for (_name, entity) in natures.iter() {
        ts::write(entity, natures, &mut dropped)?;
    }
    Ok(())
}

pub fn dts(natures: &Natures) -> Result<(), E> {
    let mut dropped: HashSet<PathBuf> = HashSet::new();
    for (_name, entity) in natures.iter() {
        dts::write(entity, natures, &mut dropped)?;
    }
    Ok(())
}

pub fn js(natures: &Natures) -> Result<(), E> {
    js::write(natures)
}
