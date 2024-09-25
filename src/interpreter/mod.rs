mod dts;
mod js;
mod offset;
mod ts;

use crate::{
    config,
    context::{Context, Target},
    error::E,
    nature::Natures,
};
pub use offset::*;
use std::{
    collections::HashSet,
    fs::{self, create_dir_all, File, OpenOptions},
    path::PathBuf,
};

pub fn serialize_name<S: AsRef<str>>(s: S) -> String {
    if s.as_ref().starts_with("r#") {
        s.as_ref().replace("r#", "")
    } else {
        s.as_ref().to_string()
    }
}

pub fn create_node_located_file(
    file_name: &str,
    dropped: &mut HashSet<PathBuf>,
) -> Result<File, E> {
    let path = config::get()?
        .node_mod_dist
        .clone()
        .ok_or(E::InvalidConfiguration(String::from(
            "No path to folder with node module. Set correct path in [tslink] of Cargo.toml; field \"node\"",
        )))?
        .join(file_name);
    if dropped.contains(&path) {
        return Ok(OpenOptions::new().append(true).open(&path)?);
    }
    if path.exists() {
        fs::remove_file(&path)?;
        let _ = dropped.insert(path.clone());
    } else if let Some(basepath) = path.parent() {
        if !basepath.exists() {
            create_dir_all(basepath)?;
        }
    } else {
        return Err(E::FileNotFound(format!(
            "Fail to get basepath from: {}",
            path.to_string_lossy()
        )));
    }
    File::create(&path)?;
    Ok(OpenOptions::new().append(true).open(&path)?)
}

pub fn create_target_file(
    context: &Context,
    target: &Target,
    dropped: &mut HashSet<PathBuf>,
) -> Result<Option<(File, PathBuf)>, E> {
    if let Some((_, path)) = context.targets.iter().find(|(t, _)| t == target) {
        if dropped.contains(path) {
            return Ok(Some((
                OpenOptions::new().append(true).open(path)?,
                path.to_owned(),
            )));
        }
        if path.exists() {
            fs::remove_file(path)?;
        } else if let Some(basepath) = path.parent() {
            if !basepath.exists() {
                create_dir_all(basepath)?;
            }
        } else {
            return Err(E::FileNotFound(format!(
                "Fail to get basepath from: {}",
                path.to_string_lossy()
            )));
        }
        let _ = dropped.insert(path.clone());
        File::create(path)?;
        Ok(Some((
            OpenOptions::new().append(true).open(path)?,
            path.to_owned(),
        )))
    } else {
        Ok(None)
    }
}

pub fn ts(natures: &Natures) -> Result<(), E> {
    let mut dropped: HashSet<PathBuf> = HashSet::new();
    for (_name, entity) in natures.iter() {
        let context = entity.get_context()?;
        if let Some((file, file_name)) = create_target_file(context, &Target::Ts, &mut dropped)? {
            let mut writer = ts::Writer::new(file, file_name);
            ts::write(entity, natures, &mut writer)?;
        }
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
