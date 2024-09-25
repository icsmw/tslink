use super::Interpreter;
use crate::{
    error::E,
    interpreter::{ts::Writer, Offset},
    nature::{Natures, Primitive},
};

impl Interpreter for Primitive {
    fn reference(
        &self,
        _natures: &Natures,
        buf: &mut Writer,
        _offset: Offset,
        _parent: Option<String>,
    ) -> Result<(), E> {
        buf.push(match self {
            Self::Number(..) => "number",
            Self::BigInt(..) => "BigInt",
            Self::String(..) => "string",
            Self::Boolean(..) => "boolean",
        });
        Ok(())
    }
}
