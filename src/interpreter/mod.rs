pub mod dts;
pub mod js;
pub mod ts;

use crate::defs::{Entities, Entity};
use std::{collections::HashSet, fmt, io::Error, path::PathBuf};

#[derive(Debug, Clone)]
pub struct Offset {
    tab: usize,
}

impl Offset {
    pub fn new() -> Self {
        Self { tab: 0 }
    }

    pub fn inc(&self) -> Self {
        Self { tab: self.tab + 1 }
    }
}

impl fmt::Display for Offset {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", " ".repeat(self.tab * 4))
    }
}

pub fn ts(entities: &Entities) -> Result<(), Error> {
    let mut dropped: HashSet<PathBuf> = HashSet::new();
    for (_name, entity) in entities.iter() {
        match entity {
            Entity::Struct(w) => ts::write(w, entities, &mut dropped)?,
            Entity::Enum(w) => ts::write(w, entities, &mut dropped)?,
            Entity::Detached(w) => ts::write(w, entities, &mut dropped)?,
        }
    }
    Ok(())
}

pub fn dts(entities: &Entities) -> Result<(), Error> {
    let mut dropped: HashSet<PathBuf> = HashSet::new();
    for (_name, entity) in entities.iter() {
        match entity {
            Entity::Struct(w) => dts::write(w, entities, &mut dropped)?,
            Entity::Enum(w) => dts::write(w, entities, &mut dropped)?,
            Entity::Detached(w) => dts::write(w, entities, &mut dropped)?,
        }
    }
    Ok(())
}

pub fn js(entities: &Entities) -> Result<(), Error> {
    js::write(entities)
}
