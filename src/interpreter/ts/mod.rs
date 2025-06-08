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

/// Trait for converting a type representation (`Nature`, `Primitive`, `Composite`, etc.)
/// into its corresponding TypeScript declaration and reference syntax.
///
/// This trait is implemented by all components that may appear as part of the TypeScript output —
/// such as classes, interfaces, function signatures, and type references.
///
/// It separates the generation into:
/// - `declaration`: full structural definition (e.g., `class MyClass { ... }`)
/// - `reference`: usage site (e.g., `arg: MyClass`)
pub trait Interpreter {
    /// Emits the full TypeScript declaration for the type.
    ///
    /// This is typically used when rendering top-level structures like:
    /// - `interface`, `class`, `type` definitions
    /// - function signatures
    /// - enums
    ///
    /// # Parameters
    /// - `natures`: The registry of all known types.
    /// - `buf`: The output writer buffer.
    /// - `offset`: Current indentation/formatting offset.
    /// - `parent`: Optional parent context (e.g., for nested members).
    fn declaration(
        &self,
        _natures: &Natures,
        _buf: &mut Writer,
        _offset: Offset,
        _parent: Option<String>,
    ) -> Result<(), E> {
        Ok(())
    }

    /// Emits a TypeScript type reference suitable for use in function arguments, fields, etc.
    ///
    /// Examples:
    /// - `MyType`
    /// - `string[]`
    /// - `Record<string, number>`
    ///
    /// # Parameters
    /// Same as `declaration`.
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

/// Delegates interpretation logic to the internal variant of `Nature`:
/// - `Primitive` types emit simple TypeScript types (e.g., `string`, `number`)
/// - `Composite` types handle containers and functions (e.g., `Array<T>`, `Result<T, E>`)
/// - `Referred` types emit named declarations (e.g., `interface MyStruct`, `class MyEnum`)
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
            Self::Referred(refered) => refered.declaration(natures, buf, offset, parent),
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
            Self::Referred(refered) => refered.reference(natures, buf, offset, parent),
        }
    }
}

/// Renders the full TypeScript declaration of a given `Interpreter` into the provided writer.
///
/// # Parameters
/// - `w`: The entity implementing `Interpreter` (typically `Nature`).
/// - `natures`: Registry of known types for cross-referencing.
/// - `buf_writer`: Output writer buffer.
///
/// # Behavior
/// - Calls `declaration(...)` on the target.
/// - Flushes the writer buffer at the end.
///
/// # Errors
/// Returns any error encountered during writing or generation.
pub fn write<T>(w: &T, natures: &Natures, buf_writer: &mut Writer) -> Result<(), E>
where
    T: Interpreter,
{
    w.declaration(natures, buf_writer, Offset::new(), None)?;
    buf_writer.write_all()?;
    Ok(())
}
