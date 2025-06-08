use crate::{
    config::cfg::EnumRepresentation, context::Context, error::E, nature::{Nature, TypeAsString, TypeTokenStream, VariableTokenStream}
};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

/// Represents named or referenced entities in a Rust codebase, typically used during TypeScript binding generation.
///
/// This enum is designed to describe named constructs such as structs, enums, fields, function arguments,
/// type aliases, and constants — capturing not only their identity (`String`), but also their context and type information.
///
/// It is used when a Rust item has a clear name in the source code and needs to be tracked, transformed, or exported
/// to a different language model (e.g., TypeScript or JSON).
///
/// Each variant represents a distinct kind of referable entity with appropriate metadata for code generation.
#[derive(Clone, Debug)]
pub enum Referred {
    /// A tuple struct declaration.
    ///
    /// - `String`: Name of the struct.
    /// - `Context`: Scope or namespace context.
    /// - `Option<Box<Nature>>`: The type of the single unnamed field (e.g., `TupleStruct(String, Context, Some(Box::new(Nature::Number)))`).
    TupleStruct(String, Context, Option<Box<Nature>>),

    /// A classic named-field struct declaration.
    ///
    /// - `String`: Name of the struct.
    /// - `Context`: Context or module where it appears.
    /// - `Vec<Nature>`: List of fields and their types.
    Struct(String, Context, Vec<Nature>),

    /// An enum type definition.
    ///
    /// - `String`: Enum name.
    /// - `Context`: Where it is defined.
    /// - `Vec<Nature>`: Enum variants.
    /// - `EnumRepresentation`: How the enum is represented (tagged, untagged, externally tagged, etc.).
    Enum(String, Context, Vec<Nature>, EnumRepresentation),

    /// A single enum variant, potentially flat (e.g., inline in a struct).
    ///
    /// - `String`: Variant name.
    /// - `Context`: Context of the enum.
    /// - `Vec<Nature>`: Fields inside this variant.
    /// - `bool`: Whether the variant is flattened (used inline).
    /// - `EnumRepresentation`: The same representation as its parent enum.
    EnumVariant(String, Context, Vec<Nature>, bool, EnumRepresentation),

    /// A named function or method.
    ///
    /// - `String`: Function name.
    /// - `Context`: Scope/module info.
    /// - `Box<Nature>`: Full type of the function (usually a `Nature::Func`).
    Func(String, Context, Box<Nature>),

    /// A named struct or enum field.
    ///
    /// - `String`: Field name.
    /// - `Context`: Struct or enum context.
    /// - `Box<Nature>`: Field type.
    /// - `Option<String>`: Optional binding name override (e.g., JS/TS alias).
    Field(String, Context, Box<Nature>, Option<String>),

    /// A named function argument.
    ///
    /// - `String`: Argument name.
    /// - `Context`: Function or method context.
    /// - `Box<Nature>`: Argument type.
    /// - `Option<String>`: Optional binding name or alias.
    FuncArg(String, Context, Box<Nature>, Option<String>),

    /// A reference to a named type.
    ///
    /// - `String`: The name being referred to.
    /// - `Option<Context>`: Where it was resolved from (if known).
    Ref(String, Option<Context>),

    /// A generic type alias (e.g., `type T = Result<i32, String>`).
    ///
    /// - `String`: Alias name.
    /// - `Box<Nature>`: Full type that this generic name refers to.
    Generic(String, Box<Nature>),

    /// A named constant definition.
    ///
    /// - `String`: Constant name.
    /// - `Context`: Location of the constant.
    /// - `Box<Nature>`: Type of the constant.
    /// - `String`: Value expression as a string (e.g., `"42"`).
    Constant(String, Context, Box<Nature>, String),
}

impl Referred {
    pub fn is_flat_varians(variants: &[Nature]) -> Result<bool, E> {
        for variant in variants {
            if let Nature::Referred(Referred::EnumVariant(_, _, values, ..)) = variant {
                if !values.is_empty() {
                    return Ok(false);
                }
            } else {
                return Err(E::Parsing(String::from("Given Nature isn't enum varian")));
            }
        }
        Ok(true)
    }

    pub fn is_enum_flat(&self) -> Result<bool, E> {
        if let Referred::Enum(_, _, variants,_) = self {
            Referred::is_flat_varians(variants)
        } else {
            Err(E::Parsing(String::from("Given Nature isn't enum")))
        }
    }
}

impl TypeTokenStream for Referred {
    fn type_token_stream(&self) -> Result<TokenStream, E> {
        let ident = match self {
            Self::TupleStruct(name, ..) => Ok(format_ident!("{}", name)),
            Self::Struct(name, ..) => Ok(format_ident!("{}", name)),
            Self::Enum(name, ..) => Ok(format_ident!("{}", name)),
            Self::Ref(name, ..) => Ok(format_ident!("{}", name)),
            Self::Constant(name, ..) => Ok(format_ident!("{}", name)),
            Self::EnumVariant(..) |
            Self::Func(..) |
            Self::Field(..) |
            Self::FuncArg(..) |
            Self::Generic(..) => Err(E::Other("EnumVariant, Func, Field, FuncArg, Generic of Referred doesn't support TypeTokenStream".to_string()))
        }?;
        Ok(quote! { #ident })
    }
}

impl TypeAsString for Referred {
    fn type_as_string(&self) -> Result<String, E> {
        match self {
            Self::TupleStruct(name, ..) => Ok(name.clone()),
            Self::Struct(name, ..) => Ok(name.clone()),
            Self::Enum(name, ..) => Ok(name.clone()),
            Self::Ref(name, ..) => Ok(name.clone()),
            Self::Constant(name, ..) => Ok(name.clone()),
            Self::EnumVariant(..) |
            Self::Func(..) |
            Self::Field(..) |
            Self::FuncArg(..) |
            Self::Generic(..) => Err(E::Other("EnumVariant, Func, Field, FuncArg, Generic of Referred doesn't support TypeAsString".to_string()))
        }
    }
}

impl VariableTokenStream for Referred {
    fn variable_token_stream(&self, var_name: &str, err: Option<&Nature>) -> Result<TokenStream, E> {
        let var_name = format_ident!("{}", var_name);
        match self {   
            Self::Ref(_, _) => {
                Ok(if let Some(nature) = err {
                    let err_type_ref = nature.type_token_stream()?;
                    quote! {
                        serde_json::to_string(&#var_name).map_err(|e| Into::<#err_type_ref>::into(e))?
                    }
                } else {
                    quote! {
                        serde_json::to_string(&#var_name).expect("Converting to JSON string")
                    }
                })
            },
            _ => {
                Err(E::Parsing(format!("Only reference to entity (struct / enum) can be convert into JSON string (var: {var_name})")))
            }
        }
    }
}

