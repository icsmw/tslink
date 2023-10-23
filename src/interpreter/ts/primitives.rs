use super::Interpreter;
use crate::{defs::Entities, interpreter::Offset, types::primitives::Primitive};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

impl Interpreter for Primitive {
    fn reference(
        &self,
        _entities: &Entities,
        buf: &mut BufWriter<File>,
        _offset: Offset,
    ) -> Result<bool, std::io::Error> {
        buf.write_all(
            match self {
                Self::Number => "number",
                Self::BigInt => "BigInt",
                Self::String => "string",
                Self::Boolean => "boolean",
            }
            .as_bytes(),
        )?;
        Ok(true)
    }
}
