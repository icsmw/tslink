use crate::{args::ArgsWriter, defs::Entities, interpreter::Offset, types::Types, CONFIG};
use std::{
    collections::HashSet,
    fs,
    fs::{File, OpenOptions},
    io::{BufWriter, Error, ErrorKind, Write},
    path::PathBuf,
};

pub mod composite;
pub mod detached;
pub mod enums;
pub mod primitives;
pub mod structs;

pub trait Interpreter {
    fn declaration(
        &self,
        _entities: &Entities,
        _buf: &mut BufWriter<File>,
        _offset: Offset,
    ) -> Result<(), Error> {
        Ok(())
    }

    fn reference(
        &self,
        _entities: &Entities,
        _buf: &mut BufWriter<File>,
        _offset: Offset,
    ) -> Result<(), Error> {
        Ok(())
    }
}

impl Interpreter for Types {
    fn reference(
        &self,
        entities: &Entities,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), Error> {
        match self {
            Self::Primitive(primitive) => primitive.reference(entities, buf, offset),
            Self::Composite(composite) => composite.reference(entities, buf, offset),
        }
    }
}

pub fn write<T>(w: &T, entities: &Entities, dropped: &mut HashSet<PathBuf>) -> Result<(), Error>
where
    T: Interpreter + ArgsWriter,
{
    let path = CONFIG
        .read()
        .map_err(|e| Error::new(ErrorKind::Other, e.to_string()))?
        .node_mod_dist
        .clone()
        .ok_or(Error::new(
            ErrorKind::Other,
            "No path to folder with node module. Set correct path in tslink.toml; field \"node\"",
        ))?
        .join("lib.d.ts");
    if !dropped.contains(&path) {
        if path.exists() {
            fs::remove_file(&path)?;
        }
        dropped.insert(path.clone());
    }
    if !path.exists() {
        File::create(&path)?;
    }
    let file = OpenOptions::new().write(true).append(true).open(&path)?;
    let mut buf_writer = BufWriter::new(file);
    w.declaration(entities, &mut buf_writer, Offset::new())?;
    buf_writer.flush()?;
    Ok(())
}
