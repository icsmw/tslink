use super::Interpreter;
use crate::{
    defs::{Entities, Entity},
    interpreter::Offset,
    types::composite::Composite,
};
use std::{
    fs::File,
    io::{BufWriter, Error, ErrorKind, Write},
};

impl Interpreter for Composite {
    fn reference(
        &self,
        entities: &Entities,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<bool, std::io::Error> {
        match self {
            Self::Vec(ty) => {
                if let Some(ty) = ty {
                    ty.reference(entities, buf, offset)?;
                    buf.write_all("[]".as_bytes())?;
                } else {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Type Vec doesn't include reference to type",
                    ));
                }
            }
            Self::HashMap(key, ty) => {
                if let (Some(key), Some(ty)) = (key, ty) {
                    buf.write_all("Map<".as_bytes())?;
                    key.reference(entities, buf, offset.clone())?;
                    buf.write_all(", ".as_bytes())?;
                    ty.reference(entities, buf, offset)?;
                    buf.write_all(">".as_bytes())?;
                } else {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Type HashMap doesn't include reference to type or key",
                    ));
                }
            }
            Self::Func(_name, args, out, asyncness) => {
                buf.write_all(format!("(").as_bytes())?;
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
            }
            Self::Tuple(tys) => {
                buf.write_all("[".as_bytes())?;
                let last = tys.len() - 1;
                for (i, ty) in tys.iter().enumerate() {
                    ty.reference(entities, buf, offset.clone())?;
                    if i < last {
                        buf.write_all(", ".as_bytes())?;
                    }
                }
                buf.write_all("]".as_bytes())?;
            }
            Self::Enum(enums) => {
                enums.reference(entities, buf, offset)?;
            }
            Self::Struct(strct) => {
                strct.reference(entities, buf, offset)?;
            }
            Self::Option(ty) => {
                if let Some(ty) = ty {
                    ty.reference(entities, buf, offset)?;
                    buf.write_all(" | undefined".as_bytes())?;
                } else {
                    return Err(Error::new(
                        ErrorKind::Other,
                        "Type Option doesn't include reference to type",
                    ));
                }
            }
            Self::RefByName(name) => {
                if let Some(entity) = entities.get(name) {
                    match entity {
                        Entity::Struct(strct) => strct.reference(entities, buf, offset)?,
                        Entity::Enum(enums) => enums.reference(entities, buf, offset)?,
                        Entity::Detached(detached) => detached.reference(entities, buf, offset)?,
                    };
                }
            }
        }
        Ok(true)
    }
}
