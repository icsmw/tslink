use super::Interpreter;
use crate::{
    error::E,
    interpreter::Offset,
    nature::{Natures, Primitive},
};
use std::{
    fs::File,
    io::{BufWriter, Write},
};

impl Interpreter for Primitive {
    fn reference(
        &self,
        _natures: &Natures,
        buf: &mut BufWriter<File>,
        _offset: Offset,
    ) -> Result<(), E> {
        Ok(buf.write_all(
            match self {
                Self::Number(_) => "number",
                Self::String => "string",
                Self::Boolean => "boolean",
            }
            .as_bytes(),
        )?)
    }
}
