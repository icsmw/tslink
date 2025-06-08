mod composite;
mod primitive;
mod refered;

use crate::{
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

/// Trait for describing how a given type (`Nature`) is declared or referenced
/// within a TypeScript declaration file (`*.d.ts`).
///
/// This trait allows recursive traversal and structured rendering of types.
/// Its implementation is simplified here, as `*.d.ts` generation doesn't need imports.
pub trait Interpreter {
    /// Writes the full type declaration, usually at the top level.
    fn declaration(
        &self,
        _natures: &Natures,
        _buf: &mut BufWriter<File>,
        _offset: Offset,
    ) -> Result<(), E> {
        Ok(())
    }

    /// Writes a reference to the type (e.g., inside a struct or function).
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
    /// Delegates declaration rendering to the specific variant:
    /// - `Primitive`, `Composite`, or `Referred`.
    fn declaration(
        &self,
        natures: &Natures,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), E> {
        match self {
            Self::Primitive(primitive) => primitive.declaration(natures, buf, offset),
            Self::Composite(composite) => composite.declaration(natures, buf, offset),
            Self::Referred(refered) => refered.declaration(natures, buf, offset),
        }
    }

    /// Same logic for references: recursively resolve how a nested type should be printed.
    fn reference(
        &self,
        natures: &Natures,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), E> {
        match self {
            Self::Primitive(primitive) => primitive.reference(natures, buf, offset),
            Self::Composite(composite) => composite.reference(natures, buf, offset),
            Self::Referred(refered) => refered.reference(natures, buf, offset),
        }
    }
}

/// Creates a `lib.d.ts` file and writes the full type declaration into it.
///
/// This is the entry point for generating global type definitions. The file is
/// created in the output directory defined by `config.node_mod_dist`. Previously
/// created files are tracked via `dropped`, ensuring clean regeneration.
///
/// # Arguments
/// - `w`: The root type or node to declare (e.g., `Nature` or any `Interpreter`).
/// - `natures`: Shared type registry, used for resolving nested types.
/// - `dropped`: Tracks written paths to prevent duplication or cleanup needs.
///
/// # Errors
/// Returns an error if file creation or writing fails.
pub fn write<T>(w: &T, natures: &Natures, dropped: &mut HashSet<PathBuf>) -> Result<(), E>
where
    T: Interpreter,
{
    let file = create_node_located_file("lib.d.ts", dropped)?;
    let mut buf_writer = BufWriter::new(file);
    w.declaration(natures, &mut buf_writer, Offset::new())?;
    buf_writer.flush()?;
    Ok(())
}
