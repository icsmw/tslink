use super::Interpreter;
use crate::{
    error::E,
    interpreter::Offset,
    nature::{Nature, Natures, Refered},
};
use std::{
    fs::File,
    io::{BufWriter, Write},
    ops::Deref,
};

impl Interpreter for Refered {
    fn declaration(
        &self,
        _natures: &Natures,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), E> {
        match self {
            Refered::Enum(name, _context, variants) => {
                buf.write_all(format!("{}exports.{name} = Object.freeze({{\n", offset).as_bytes())?;
                for (i, variant) in variants.iter().enumerate() {
                    if let Nature::Refered(Refered::EnumVariant(name, _, _, _)) = variant.deref() {
                        buf.write_all(
                            format!("{}{name}: {i}, \"{i}\": \"{name}\",\n", offset.inc())
                                .as_bytes(),
                        )?;
                    } else {
                        return Err(E::Parsing(String::from(
                            "Given nature isn't Enum's variant",
                        )));
                    }
                }
                buf.write_all(format!("{}}});\n", offset).as_bytes())?;
            }
            _ => {
                return Err(E::Parsing(format!(
                    "Given nature cannot be declared for JS"
                )));
            }
        }
        Ok(())
    }
}
