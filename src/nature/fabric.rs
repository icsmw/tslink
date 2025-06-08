use crate::{error::E, nature::Nature};
use proc_macro2::TokenStream;

pub trait TypeTokenStream {
    /// A trait for extracting a Rust type representation as a [`TokenStream`].
    ///
    /// This trait is primarily used when modifying Rust code during procedural macro execution.
    /// It allows implementers to provide a syntax-level representation of a type (e.g., `String`, `i32`, or even complex types),
    /// which is particularly useful when wrapping function arguments or return types,
    /// such as converting them to/from JSON or bridging them to JavaScript/TypeScript.
    ///
    /// # Use Cases
    ///
    /// - Generating Rust wrapper functions that serialize arguments or return values.
    /// - Producing accurate and editable Rust code during macro expansion.
    /// - Converting internal type representations into valid Rust syntax tree elements.
    ///
    /// # Errors
    ///
    /// Implementations may return an error if the type cannot be converted into a valid token stream.
    fn type_token_stream(&self) -> Result<TokenStream, E>;
}

pub trait TypeAsString {
    /// A trait for extracting the string representation of a type as it should appear in TypeScript.
    ///
    /// This trait is used during the generation of `.ts` or `.d.ts` files,
    /// where a human-readable TypeScript-compatible string is required to describe the type.
    ///
    /// Typical use cases include:
    /// - Mapping Rust types to their corresponding TypeScript forms (e.g., `i32` → `"number"`, `String` → `"string"`).
    /// - Generating type signatures for function arguments and return values.
    /// - Writing interface definitions or type declarations in emitted TypeScript files.
    ///
    /// # Errors
    ///
    /// Returns an error if the type cannot be represented as a valid TypeScript string.
    fn type_as_string(&self) -> Result<String, E>;
}

pub trait VariableTokenStream {
    /// A trait for generating a `TokenStream` representing how a variable should be processed in Rust code.
    ///
    /// Similar to [`TypeTokenStream`], this trait is used during procedural macro transformations
    /// to emit Rust code that manipulates variables — for example, wrapping or converting them for
    /// JSON serialization/deserialization, argument mapping, or runtime checks.
    ///
    /// # Parameters
    ///
    /// - `var_name`: The name of the variable in Rust code as a string.
    /// - `err`: An optional reference to a [`Nature`] value, which may influence how errors or special cases
    ///   are handled during code generation.
    ///
    /// # Use Cases
    ///
    /// - Generating Rust code for transforming function arguments before invoking the actual logic.
    /// - Inserting serialization logic for return values or parameters.
    /// - Applying custom error handling or wrappers during macro expansion.
    ///
    /// # Errors
    ///
    /// Returns an error if the generated code cannot be constructed or if the variable transformation is unsupported.
    fn variable_token_stream(&self, var_name: &str, err: Option<&Nature>)
        -> Result<TokenStream, E>;
}

impl TypeTokenStream for Nature {
    fn type_token_stream(&self) -> Result<TokenStream, E> {
        match self {
            Nature::Composite(ty) => ty.type_token_stream(),
            Nature::Primitive(ty) => ty.type_token_stream(),
            Nature::Referred(ty) => ty.type_token_stream(),
        }
    }
}

impl TypeAsString for Nature {
    fn type_as_string(&self) -> Result<String, E> {
        match self {
            Nature::Composite(ty) => ty.type_as_string(),
            Nature::Primitive(ty) => ty.type_as_string(),
            Nature::Referred(ty) => ty.type_as_string(),
        }
    }
}

impl VariableTokenStream for Nature {
    fn variable_token_stream(
        &self,
        var_name: &str,
        err: Option<&Nature>,
    ) -> Result<TokenStream, E> {
        match self {
            Nature::Composite(v) => v.variable_token_stream(var_name, err),
            Nature::Primitive(v) => v.variable_token_stream(var_name, err),
            Nature::Referred(v) => v.variable_token_stream(var_name, err),
        }
    }
}
