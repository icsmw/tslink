use super::*;
use crate::defs::{enums::Enums, structs::Structs};

#[derive(Clone, Debug)]
pub enum Composite {
    Vec(Option<Box<Types>>),
    HashMap(Option<primitives::Primitive>, Option<Box<Types>>),
    // (Name, Vec(ArgName, ArgType), Output, async)
    Func(String, Vec<(String, Box<Types>)>, Option<Box<Types>>, bool),
    Tuple(Vec<Box<Types>>),
    Option(Option<Box<Types>>),
    Struct(Structs),
    Enum(Enums),
    RefByName(String),
}
