use super::Interpreter;
use crate::{
    error::E,
    interpreter::{ts::Writer, Offset},
    nature::{Natures, Primitive},
};

/// Implements `Interpreter` for `Primitive` types by generating their corresponding
/// TypeScript type literals during type reference rendering.
///
/// This trait implementation is used when rendering **inlined references** to primitive types
/// (e.g., in function signatures, field declarations, etc.).
///
/// # Type Mapping
/// | Rust `Primitive` variant | TypeScript equivalent |
/// |--------------------------|------------------------|
/// | `Number`                 | `number`               |
/// | `BigInt`                 | `BigInt`               |
/// | `String`                 | `string`               |
/// | `Boolean`                | `boolean`              |
///
/// # Behavior
/// - Only `reference()` is implemented. Primitives are not declared, only referenced.
/// - Appends the appropriate TypeScript primitive keyword to the buffer.
///
/// # Parameters
/// - `_natures`: Ignored (not used for primitives)
/// - `buf`: Output writer receiving the emitted TypeScript string
/// - `_offset`: Ignored (no formatting needed for primitives)
/// - `_parent`: Ignored (primitives are global types)
///
/// # Example Output
/// - `number`
/// - `BigInt`
/// - `string`
/// - `boolean`
impl Interpreter for Primitive {
    fn reference(
        &self,
        _natures: &Natures,
        buf: &mut Writer,
        _offset: Offset,
        _parent: Option<String>,
    ) -> Result<(), E> {
        buf.push(match self {
            Self::Number(..) => "number",
            Self::BigInt(..) => "BigInt",
            Self::String(..) => "string",
            Self::Boolean(..) => "boolean",
        });
        Ok(())
    }
}
