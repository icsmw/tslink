use super::Interpreter;
use crate::{
    defs::{detached::Detached, Entities},
    interpreter::Offset,
    types::{composite::Composite, Types},
};
use std::{
    fs::File,
    io::{BufWriter, Error, ErrorKind, Write},
};

impl Interpreter for Detached {
    fn declaration(
        &self,
        entities: &Entities,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), Error> {
        match &self.ty {
            Types::Composite(composite) => match composite {
                Composite::Func(name, args, out, asyncness) => {
                    buf.write_all(format!("{}export declare function {name}(", offset).as_bytes())?;
                    for (i, (name, ty)) in args.iter().enumerate() {
                        buf.write_all(format!("{name}: ").as_bytes())?;
                        ty.reference(entities, buf, offset.clone())?;
                        if i < args.len() - 1 {
                            buf.write_all(", ".as_bytes())?;
                        }
                    }
                    buf.write_all(format!("): ").as_bytes())?;
                    if *asyncness {
                        buf.write_all(format!("Promise<").as_bytes())?;
                    }
                    if let Some(out) = out {
                        out.reference(entities, buf, offset.clone())?;
                    } else {
                        buf.write_all(format!("void").as_bytes())?;
                    }
                    if *asyncness {
                        buf.write_all(format!(">").as_bytes())?;
                    }
                    buf.write_all(format!(";\n").as_bytes())
                }
                _ => Err(Error::new(
                    ErrorKind::Other,
                    "Only functions are supported as detached types".to_string(),
                )),
            },
            Types::Primitive(_) => Err(Error::new(
                ErrorKind::Other,
                "Primitive types arn't support as detached types".to_string(),
            )),
        }
    }
    fn reference(
        &self,
        _entities: &Entities,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), Error> {
        match &self.ty {
            Types::Composite(composite) => match composite {
                Composite::Func(name, _args, _out, _asyncness) => {
                    buf.write_all(format!("{}{name}", offset).as_bytes())
                }
                _ => Err(Error::new(
                    ErrorKind::Other,
                    "Only functions are supported as detached types".to_string(),
                )),
            },
            Types::Primitive(_) => Err(Error::new(
                ErrorKind::Other,
                "Primitive types arn't support as detached types".to_string(),
            )),
        }
    }
}
