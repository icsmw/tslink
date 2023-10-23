use super::Interpreter;
use crate::{
    defs::{
        enums::{Enums, Variant},
        Entities,
    },
    interpreter::Offset,
};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

impl Interpreter for Variant {
    fn declaration(
        &self,
        entities: &Entities,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), std::io::Error> {
        buf.write_all(format!("{}{}: ", offset, self.name).as_bytes())?;
        if self.fields.is_empty() {
            buf.write_all(format!("null").as_bytes())?;
        } else if self.fields.len() == 1 {
            self.fields
                .first()
                .expect("Expecting single field for Variant")
                .reference(entities, buf, offset.inc())?;
        } else {
            buf.write_all(format!(" [").as_bytes())?;
            for (i, field) in self.fields.iter().enumerate() {
                field.reference(entities, buf, offset.inc())?;
                if i < self.fields.len() - 1 {
                    buf.write_all(format!(", ").as_bytes())?;
                }
            }
            buf.write_all(format!("]").as_bytes())?;
        }
        buf.write_all(format!(" | undefined").as_bytes())?;
        Ok(())
    }
    fn reference(
        &self,
        _entities: &Entities,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), std::io::Error> {
        buf.write_all(format!("{}{}: ", offset, self.name).as_bytes())
    }
}

impl Interpreter for Enums {
    fn declaration(
        &self,
        entities: &Entities,
        buf: &mut BufWriter<File>,
        offset: Offset,
    ) -> Result<(), std::io::Error> {
        buf.write_all(format!("{}{} {} {{\n", offset, "export interface", self.name).as_bytes())?;
        for variant in self.variants.iter() {
            variant.declaration(entities, buf, offset.inc())?;
            buf.write_all(format!(";\n").as_bytes())?;
        }
        buf.write_all(format!("{}}}\n", offset).as_bytes())?;
        Ok(())
    }
    fn reference(
        &self,
        _entities: &Entities,
        buf: &mut BufWriter<File>,
        _offset: Offset,
    ) -> Result<(), std::io::Error> {
        buf.write_all(format!("{}", self.name).as_bytes())
    }
}
