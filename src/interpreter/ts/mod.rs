mod composite;
mod export;
mod import;
mod indexer;
mod primitive;
mod refered;
mod writer;

pub use export::*;
pub use import::*;
pub use indexer::*;
pub use writer::*;

use crate::{
    error::E,
    interpreter::Offset,
    nature::{Nature, Natures},
};

pub trait Interpreter {
    fn declaration(
        &self,
        _natures: &Natures,
        _buf: &mut Writer,
        _offset: Offset,
        _parent: Option<String>,
    ) -> Result<(), E> {
        Ok(())
    }

    fn reference(
        &self,
        _natures: &Natures,
        _buf: &mut Writer,
        _offset: Offset,
        _parent: Option<String>,
    ) -> Result<(), E> {
        Ok(())
    }
}

impl Interpreter for Nature {
    fn declaration(
        &self,
        natures: &Natures,
        buf: &mut Writer,
        offset: Offset,
        parent: Option<String>,
    ) -> Result<(), E> {
        match self {
            Self::Primitive(primitive) => primitive.declaration(natures, buf, offset, parent),
            Self::Composite(composite) => composite.declaration(natures, buf, offset, parent),
            Self::Refered(refered) => refered.declaration(natures, buf, offset, parent),
        }
    }

    fn reference(
        &self,
        natures: &Natures,
        buf: &mut Writer,
        offset: Offset,
        parent: Option<String>,
    ) -> Result<(), E> {
        match self {
            Self::Primitive(primitive) => primitive.reference(natures, buf, offset, parent),
            Self::Composite(composite) => composite.reference(natures, buf, offset, parent),
            Self::Refered(refered) => refered.reference(natures, buf, offset, parent),
        }
    }
}

pub fn write<T>(w: &T, natures: &Natures, buf_writer: &mut Writer) -> Result<(), E>
where
    T: Interpreter,
{
    w.declaration(natures, buf_writer, Offset::new(), None)?;
    buf_writer.write_all()?;
    Ok(())
}
