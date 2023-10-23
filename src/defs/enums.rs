use crate::{
    args::{Args, ArgsWriter},
    types::Types,
};
use syn::{punctuated::Punctuated, token::Comma, Fields};

#[derive(Clone, Debug)]
pub struct Variant {
    pub name: String,
    pub fields: Vec<Types>,
    pub args: Args,
}

impl ArgsWriter for Variant {
    fn get_args(&self) -> &Args {
        &self.args
    }
}

impl Variant {
    pub fn new(name: &str, args: Args) -> Self {
        Self {
            name: name.to_string(),
            fields: vec![],
            args,
        }
    }
    pub fn read(&mut self, fields: &Fields) -> Result<(), String> {
        match fields {
            Fields::Named(ref fields) => {
                for field in fields.named.iter() {
                    // let name = field.ident.clone().unwrap();
                    self.fields.push(Types::from(&field.ty));
                }
            }
            Fields::Unnamed(ref fields) => {
                for field in fields.unnamed.iter() {
                    self.fields.push(Types::from(&field.ty));
                }
            }
            Fields::Unit => {}
        }
        Ok(())
    }
}

#[derive(Clone, Debug)]
pub struct Enums {
    pub variants: Vec<Variant>,
    pub args: Args,
    pub name: String,
}

impl ArgsWriter for Enums {
    fn get_args(&self) -> &Args {
        &self.args
    }
}

impl Enums {
    pub fn new(name: &str, args: Args) -> Self {
        Self {
            variants: vec![],
            name: name.to_string(),
            args,
        }
    }
    pub fn read(&mut self, variants: &Punctuated<syn::Variant, Comma>) -> Result<(), String> {
        for variant in variants {
            let name = variant.ident.to_string();
            if self.args.is_ignored(&name) {
                continue;
            }
            let mut v = Variant::new(&name, self.args.clone());
            v.read(&variant.fields)?;
            if self.variants.iter().any(|v| v.name == name) {
                return Err(format!("Enum Variant \"{}\" already exists", name));
            }
            self.variants.push(v);
        }
        Ok(())
    }
    pub fn is_flat(&self) -> bool {
        !self
            .variants
            .iter()
            .any(|variant| !variant.fields.is_empty())
    }
}
