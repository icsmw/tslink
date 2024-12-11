use crate::{
    config::cfg::EnumRepresentation, context::Context, error::E, nature::{Nature, TypeAsString, TypeTokenStream, VariableTokenStream}
};
use proc_macro2::TokenStream;
use quote::{quote, format_ident};

#[derive(Clone, Debug)]
pub enum Refered {
    // Name, Context, Field
    TupleStruct(String, Context, Option<Box<Nature>>),
    // Name, Context, Fields
    Struct(String, Context, Vec<Nature>),
    // Name, Context, Variants, EnumRepresentation
    Enum(String, Context, Vec<Nature>, EnumRepresentation),
    // name, context, values, is_flat
    EnumVariant(String, Context, Vec<Nature>, bool, EnumRepresentation),
    // Name, Context, FuncNature
    Func(String, Context, Box<Nature>),
    // Name, Context, Nature, Binding
    Field(String, Context, Box<Nature>, Option<String>),
    // Name, Context, Nature, Binding
    FuncArg(String, Context, Box<Nature>, Option<String>),
    // Name
    Ref(String, Option<Context>),
    // Alias, Nature
    Generic(String, Box<Nature>)
}

impl Refered {
    pub fn is_flat_varians(variants: &[Nature]) -> Result<bool, E> {
        for variant in variants {
            if let Nature::Refered(Refered::EnumVariant(_, _, values, ..)) = variant {
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
        if let Refered::Enum(_, _, variants,_) = self {
            Refered::is_flat_varians(variants)
        } else {
            Err(E::Parsing(String::from("Given Nature isn't enum")))
        }
    }
}

impl TypeTokenStream for Refered {
    fn type_token_stream(&self) -> Result<TokenStream, E> {
        let ident = match self {
            Self::TupleStruct(name, ..) => Ok(format_ident!("{}", name)),
            Self::Struct(name, ..) => Ok(format_ident!("{}", name)),
            Self::Enum(name, ..) => Ok(format_ident!("{}", name)),
            Self::Ref(name, ..) => Ok(format_ident!("{}", name)),
            Self::EnumVariant(..) |
            Self::Func(..) |
            Self::Field(..) |
            Self::FuncArg(..) |
            Self::Generic(..) => Err(E::Other("EnumVariant, Func, Field, FuncArg, Generic of Refered doesn't support TypeTokenStream".to_string()))
        }?;
        Ok(quote! { #ident })
    }
}

impl TypeAsString for Refered {
    fn type_as_string(&self) -> Result<String, E> {
        match self {
            Self::TupleStruct(name, ..) => Ok(name.clone()),
            Self::Struct(name, ..) => Ok(name.clone()),
            Self::Enum(name, ..) => Ok(name.clone()),
            Self::Ref(name, ..) => Ok(name.clone()),
            Self::EnumVariant(..) |
            Self::Func(..) |
            Self::Field(..) |
            Self::FuncArg(..) |
            Self::Generic(..) => Err(E::Other("EnumVariant, Func, Field, FuncArg, Generic of Refered doesn't support TypeAsString".to_string()))
        }
    }
}

impl VariableTokenStream for Refered {
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

