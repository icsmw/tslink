mod composite;
mod primitive;
mod refered;

use crate::{
    config,
    error::E,
    interpreter::{create_node_located_file, Offset},
    nature::{Nature, Natures},
};
use std::{
    collections::HashSet,
    fs::File,
    io::{BufWriter, Write},
    path::PathBuf,
};

pub trait Interpreter {
    fn declaration(
        &self,
        _natures: &Natures,
        _buf: &mut BufWriter<File>,
        _offset: Offset,
    ) -> Result<(), E> {
        Ok(())
    }

    fn reference(
        &self,
        _natures: &Natures,
        _buf: &mut BufWriter<File>,
        _offset: Offset,
    ) -> Result<(), E> {
        Ok(())
    }
}

impl Interpreter for Nature {
    fn declaration(
        &self,
        natures: &Natures,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), E> {
        match self {
            Self::Primitive(primitive) => primitive.declaration(natures, buf, offset),
            Self::Composite(composite) => composite.declaration(natures, buf, offset),
            Self::Refered(refered) => refered.declaration(natures, buf, offset),
        }
    }

    fn reference(
        &self,
        natures: &Natures,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), E> {
        match self {
            Self::Primitive(primitive) => primitive.reference(natures, buf, offset),
            Self::Composite(composite) => composite.reference(natures, buf, offset),
            Self::Refered(refered) => refered.reference(natures, buf, offset),
        }
    }
}

pub fn write<T>(w: &T, natures: &Natures, dropped: &mut HashSet<PathBuf>) -> Result<(), E>
where
    T: Interpreter,
{
    let file = create_node_located_file("lib.ts", dropped)?;
    let mut buf_writer = BufWriter::new(file);
    w.declaration(natures, &mut buf_writer, Offset::new())?;
    buf_writer.flush()?;
    Ok(())
}
